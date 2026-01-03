// src-tauri/src/dhcp.rs

use serde::{Serialize, Deserialize};
use crate::sysmodules::{fetch, post, logging, config};
use dotenv::var;
use std::net::UdpSocket;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

static DHCP_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lease {
    pub ip: String,
    pub mac: String,
    pub hostname: String,
    pub expires_at: String, // ISO timestamp
    pub static_lease: bool,
}

#[derive(Deserialize)]
pub struct LeaseInput {
    pub ip: String,
    pub mac: String,
    pub hostname: String,
}

fn get_leases_file() -> String {
    var("CRABFLOW_DHCP_CONFIG").unwrap_or_else(|_| "leases.json".to_string())
}

/// List all DHCP leases
pub fn list_leases() -> Vec<Lease> {
    // Example: read from JSON file using fetch.rs
    match fetch::read_file(&get_leases_file()) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => vec![],
    }
}

/// Add a static lease
pub fn add_static_lease(input: LeaseInput) -> Result<(), String> {
    let mut leases = list_leases();
    let ip_for_log = input.ip.clone();
    
    // Remove existing lease for this IP if any
    leases.retain(|l| l.ip != input.ip);
    
    leases.push(Lease {
        ip: input.ip,
        mac: input.mac,
        hostname: input.hostname,
        expires_at: "never".into(),
        static_lease: true,
    });

    let serialized = serde_json::to_string_pretty(&leases).map_err(|e| e.to_string())?;
    post::write_file(&get_leases_file(), &serialized)?;
    logging::log_event("system".into(), "add_static_lease".into(), ip_for_log);
    Ok(())
}

/// Remove a lease by IP
pub fn remove_lease(ip: String) -> Result<(), String> {
    let mut leases = list_leases();
    leases.retain(|l| l.ip != ip);

    let serialized = serde_json::to_string_pretty(&leases).map_err(|e| e.to_string())?;
    post::write_file(&get_leases_file(), &serialized)?;
    logging::log_event("system".into(), "remove_lease".into(), ip);
    Ok(())
}

// --- DHCP Server Implementation ---

pub fn stop_dhcp_server() {
    if DHCP_RUNNING.load(Ordering::Relaxed) {
        logging::log_info("Stopping DHCP Server...");
        DHCP_RUNNING.store(false, Ordering::Relaxed);
    }
}

pub fn start_dhcp_server() {
    if DHCP_RUNNING.load(Ordering::Relaxed) {
        logging::log_info("DHCP Server is already running.");
        return;
    }

    DHCP_RUNNING.store(true, Ordering::Relaxed);

    thread::spawn(|| {
        let socket = match UdpSocket::bind("0.0.0.0:67") {
            Ok(s) => s,
            Err(e) => {
                logging::log_error(&format!("Failed to bind DHCP server to port 67: {}", e));
                DHCP_RUNNING.store(false, Ordering::Relaxed);
                return;
            }
        };
        
        socket.set_broadcast(true).unwrap_or_else(|e| logging::log_error(&format!("Failed to set broadcast: {}", e)));
        socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap_or_else(|e| logging::log_error(&format!("Failed to set read timeout: {}", e)));
        
        logging::log_info("DHCP Server started on port 67");

        let mut buf = [0u8; 1500];
        while DHCP_RUNNING.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let packet = &buf[..amt];
                    handle_dhcp_packet(&socket, packet, src);
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock && e.kind() != std::io::ErrorKind::TimedOut {
                        logging::log_error(&format!("Error receiving DHCP packet: {}", e));
                    }
                }
            }
        }
        logging::log_info("DHCP Server stopped.");
    });
}

fn handle_dhcp_packet(socket: &UdpSocket, packet: &[u8], _src: std::net::SocketAddr) {
    // Basic validation (Op=1 BootRequest, Hlen=6, Magic Cookie)
    if packet.len() < 240 || packet[0] != 1 || packet[2] != 6 {
        return;
    }
    
    // Check Magic Cookie (99, 130, 83, 99)
    if packet[236] != 99 || packet[237] != 130 || packet[238] != 83 || packet[239] != 99 {
        return;
    }

    // Extract Transaction ID (XID)
    let xid = &packet[4..8];
    // Extract Client MAC (CHADDR)
    let mac_bytes = &packet[28..34];
    let mac_str = format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", 
        mac_bytes[0], mac_bytes[1], mac_bytes[2], mac_bytes[3], mac_bytes[4], mac_bytes[5]);

    // Parse Options
    let mut msg_type = 0;
    let mut hostname = "Unknown".to_string();
    
    let mut i = 240;
    while i < packet.len() {
        let opt_code = packet[i];
        if opt_code == 255 { break; } // End option
        if opt_code == 0 { i += 1; continue; } // Pad option
        
        let opt_len = packet[i+1] as usize;
        let opt_val = &packet[i+2..i+2+opt_len];
        
        match opt_code {
            53 => msg_type = opt_val[0], // DHCP Message Type
            12 => hostname = String::from_utf8_lossy(opt_val).to_string(), // Hostname
            _ => {}
        }
        
        i += 2 + opt_len;
    }

    // Load Config
    let setup = match fetch::fetch_setup() {
        Ok(s) => s,
        Err(_) => return, // Can't serve without config
    };
    
    if !setup.dhcp.enabled {
        return;
    }

    // Logic for DISCOVER (1) and REQUEST (3)
    if msg_type == 1 || msg_type == 3 {
        let offered_ip = allocate_ip(&mac_str, &hostname, &setup.dhcp);
        
        if let Some(ip) = offered_ip {
            let response_type = if msg_type == 1 { 2 } else { 5 }; // OFFER or ACK
            send_dhcp_reply(socket, packet, xid, &ip, &mac_bytes, response_type, &setup.dhcp);
            
            if response_type == 5 {
                logging::log_info(&format!("DHCP ACK sent to {} ({})", ip, mac_str));
            } else {
                logging::log_info(&format!("DHCP OFFER sent to {} ({})", ip, mac_str));
            }
        }
    }
}

fn allocate_ip(mac: &str, hostname: &str, config: &config::DhcpConfig) -> Option<String> {
    let mut leases = list_leases();
    
    // 1. Check if MAC already has a lease
    if let Some(lease) = leases.iter().find(|l| l.mac == mac) {
        return Some(lease.ip.clone());
    }
    
    // 2. Find next available IP
    let start_parts: Vec<u8> = config.range_start.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let end_parts: Vec<u8> = config.range_end.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    
    if start_parts.len() != 4 || end_parts.len() != 4 { return None; }
    
    let mut current = start_parts.clone();
    
    while current <= end_parts {
        let current_ip = format!("{}.{}.{}.{}", current[0], current[1], current[2], current[3]);
        
        if !leases.iter().any(|l| l.ip == current_ip) {
            // Found free IP, create lease
            let expires = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + config.lease_time;
            
            leases.push(Lease {
                ip: current_ip.clone(),
                mac: mac.to_string(),
                hostname: hostname.to_string(),
                expires_at: expires.to_string(),
                static_lease: false,
            });
            
            // Save leases
            if let Ok(serialized) = serde_json::to_string_pretty(&leases) {
                let _ = post::write_file(&get_leases_file(), &serialized);
            }
            
            return Some(current_ip);
        }
        
        // Increment IP
        for i in (0..4).rev() {
            if current[i] < 255 {
                current[i] += 1;
                break;
            } else {
                current[i] = 0;
            }
        }
    }
    
    None // Pool exhausted
}

fn send_dhcp_reply(socket: &UdpSocket, req: &[u8], xid: &[u8], yiaddr: &str, chaddr: &[u8], msg_type: u8, config: &config::DhcpConfig) {
    let mut packet = vec![0u8; 300];
    
    packet[0] = 2; // BootReply
    packet[1] = 1; // Ethernet
    packet[2] = 6; // Hlen
    packet[3] = 0; // Hops
    
    // XID
    packet[4..8].copy_from_slice(xid);
    
    // YIADDR (Your IP)
    let ip_parts: Vec<u8> = yiaddr.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    packet[16..20].copy_from_slice(&ip_parts);
    
    // SIADDR (Server IP - Gateway)
    let server_ip: Vec<u8> = config.gateway.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    packet[20..24].copy_from_slice(&server_ip);
    
    // CHADDR (Client MAC)
    packet[28..34].copy_from_slice(chaddr);
    
    // Magic Cookie
    packet[236] = 99; packet[237] = 130; packet[238] = 83; packet[239] = 99;
    
    let mut i = 240;
    
    // Option 53: Message Type
    packet[i] = 53; packet[i+1] = 1; packet[i+2] = msg_type; i += 3;
    
    // Option 54: Server Identifier
    packet[i] = 54; packet[i+1] = 4; 
    packet[i+2..i+6].copy_from_slice(&server_ip); i += 6;
    
    // Option 51: Lease Time
    let lease_bytes = (config.lease_time as u32).to_be_bytes();
    packet[i] = 51; packet[i+1] = 4;
    packet[i+2..i+6].copy_from_slice(&lease_bytes); i += 6;
    
    // Option 1: Subnet Mask
    let mask_parts: Vec<u8> = config.subnet_mask.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    packet[i] = 1; packet[i+1] = 4;
    packet[i+2..i+6].copy_from_slice(&mask_parts); i += 6;
    
    // Option 3: Router (Gateway)
    packet[i] = 3; packet[i+1] = 4;
    packet[i+2..i+6].copy_from_slice(&server_ip); i += 6;
    
    // Option 6: DNS Servers
    let mut dns_bytes = Vec::new();
    
    // If Captive Portal is enabled, force DNS to be the Gateway (us)
    let dns_list = if config.captive_portal {
        vec![config.gateway.clone()]
    } else {
        config.dns_servers.clone()
    };

    for dns in &dns_list {
        let parts: Vec<u8> = dns.split('.').map(|s| s.parse().unwrap_or(0)).collect();
        dns_bytes.extend_from_slice(&parts);
    }
    if !dns_bytes.is_empty() {
        packet[i] = 6; packet[i+1] = dns_bytes.len() as u8;
        packet[i+2..i+2+dns_bytes.len()].copy_from_slice(&dns_bytes);
        i += 2 + dns_bytes.len();
    }
    
    // End Option
    packet[i] = 255;
    
    // Send to broadcast (255.255.255.255:68)
    let _ = socket.send_to(&packet[..i+1], "255.255.255.255:68");
}
