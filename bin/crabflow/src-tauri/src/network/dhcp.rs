// src-tauri/src/dhcp.rs

use serde::{Serialize, Deserialize};
use crate::sysmodules::{fetch, post, logging, config, notify, paths};
use tauri::AppHandle;
use std::net::UdpSocket;
use std::thread;
// use std::sync::{Arc, Mutex};
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
    paths::get_config_path("leases.json").to_string_lossy().to_string()
}

/// List all DHCP leases
pub fn list_leases() -> Vec<Lease> {
    // Example: read from JSON file using fetch.rs
    match fetch::read_file(&get_leases_file()) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => vec![],
    }
}

/// Check if an IP is actively leased
pub fn is_ip_leased(ip: &str) -> bool {
    let leases = list_leases();
    // Simple check: is IP in the list and not expired? 
    // For now just check existence.
    leases.iter().any(|l| l.ip == ip)
}

/// Retrieve the MAC address for a given IP from the active leases
pub fn get_mac_from_ip(target_ip: &str) -> Option<String> {
    let leases = list_leases();
    leases.into_iter()
        .find(|l| l.ip == target_ip)
        .map(|l| l.mac)
}

#[tauri::command]
pub fn get_next_free_ip() -> Option<String> {
    let cfg = match config::load_setup_config() {
        Ok(c) => c,
        Err(_) => return None,
    };
    let dh = cfg.dhcp;
    let mut leases = list_leases();

    let start_parts: Vec<u8> = dh.range_start.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let end_parts: Vec<u8> = dh.range_end.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    if start_parts.len() != 4 || end_parts.len() != 4 { return None; }

    let mut current = start_parts.clone();
    while current <= end_parts {
        let current_ip = format!("{}.{}.{}.{}", current[0], current[1], current[2], current[3]);
        if current_ip == dh.gateway || current_ip == dh.bind_address {
            for i in (0..4).rev() { if current[i] < 255 { current[i] += 1; break; } else { current[i] = 0; } }
            continue;
        }
        if !leases.iter().any(|l| l.ip == current_ip) {
            return Some(current_ip);
        }
        for i in (0..4).rev() { if current[i] < 255 { current[i] += 1; break; } else { current[i] = 0; } }
    }
    None
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

pub fn is_server_running() -> bool {
    DHCP_RUNNING.load(Ordering::Relaxed)
}

pub fn start_dhcp_server(app_handle: Option<AppHandle>) {
    if DHCP_RUNNING.load(Ordering::Relaxed) {
        logging::log_info("DHCP Server is already running.");
        return;
    }

    // Load config to check if enabled and get bind address
    let config = match config::load_setup_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            logging::log_error(&format!("Failed to load config for DHCP: {}", e));
            return;
        }
    };

    if !config.dhcp.enabled {
        logging::log_info("DHCP Server is disabled in config. Skipping start.");
        return;
    }

    DHCP_RUNNING.store(true, Ordering::Relaxed);
    let app = app_handle.clone();

    thread::spawn(move || {
        let bind_addr_raw = config.dhcp.bind_address.clone();
        // Resolve bind address: it may be an IP, '0.0.0.0', or an interface name
        let bind_ip = if bind_addr_raw == "0.0.0.0" {
            "0.0.0.0".to_string()
        } else {
            // Try parse as IP first
            if bind_addr_raw.parse::<std::net::IpAddr>().is_ok() {
                bind_addr_raw.clone()
            } else {
                // Treat as interface name: find first IPv4 on that interface
                let mut resolved = "0.0.0.0".to_string();
                if let Ok(ifaces) = get_if_addrs::get_if_addrs() {
                    for iface in ifaces {
                        if iface.name == bind_addr_raw {
                            let ip = iface.addr.ip();
                            if ip.is_ipv4() {
                                resolved = ip.to_string();
                                break;
                            }
                        }
                    }
                }
                resolved
            }
        };
        let bind_endpoint = format!("{}:67", bind_ip);

        let socket = match UdpSocket::bind(&bind_endpoint) {
            Ok(s) => s,
            Err(e) => {
                let err_msg = format!("Failed to bind DHCP server to {}: {}", bind_endpoint, e);
                logging::log_error(&err_msg);
                if let Some(h) = &app {
                    notify::send_notification(h, "DHCP Error", &err_msg, "error");
                }
                DHCP_RUNNING.store(false, Ordering::Relaxed);
                return;
            }
        };
        
        socket.set_broadcast(true).unwrap_or_else(|e| logging::log_error(&format!("Failed to set broadcast: {}", e)));
        socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap_or_else(|e| logging::log_error(&format!("Failed to set read timeout: {}", e)));
        
        logging::log_info(&format!("DHCP Server started on {}", bind_endpoint));
        if let Some(h) = &app {
            notify::send_notification(h, "DHCP Started", &format!("Listening on {}", bind_endpoint), "success");
        }

        // Determine server MAC for the bind address to avoid leasing ourselves
        let server_mac = {
            let mut nets = sysinfo::Networks::new_with_refreshed_list();
            nets.refresh();
            let mut found: Option<String> = None;
            for (_name, data) in nets.iter() {
                let mac = data.mac_address().to_string();
                if !mac.is_empty() {
                    if found.is_none() { found = Some(mac.clone()); }
                }
            }
            found
        };

        let mut buf = [0u8; 1500];
        while DHCP_RUNNING.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let packet = &buf[..amt];
                    logging::log_debug(&format!("DHCP packet received from {} ({} bytes)", src, amt));
                    handle_dhcp_packet(&socket, packet, src, &app, server_mac.clone());
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

fn handle_dhcp_packet(socket: &UdpSocket, packet: &[u8], _src: std::net::SocketAddr, app: &Option<AppHandle>, server_mac: Option<String>) {
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
    let mut requested_ip: Option<String> = None;
    
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
            50 => { // Requested IP Address
                if opt_val.len() == 4 {
                    requested_ip = Some(format!("{}.{}.{}.{}", opt_val[0], opt_val[1], opt_val[2], opt_val[3]));
                }
            }
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

    // If the packet is from the server's own MAC, ignore it (prevents the server getting a lease)
    if let Some(ref smac) = server_mac {
        if !smac.is_empty() {
            if mac_str.to_lowercase() == smac.to_lowercase() {
                logging::log_debug(&format!("Ignoring DHCP packet from server MAC {}", smac));
                return;
            }
        }
    }

    // Logic for DISCOVER (1) and REQUEST (3)
    if msg_type == 1 || msg_type == 3 {
        logging::log_debug(&format!("DHCP message type {} received from MAC {}", msg_type, mac_str));

        if msg_type == 1 {
            // DISCOVER -> Offer an IP but do NOT persist the lease yet
            let offered_ip = find_free_ip(&setup.dhcp, Some(&mac_str));
            if let Some(ip) = offered_ip {
                send_dhcp_reply(socket, packet, xid, &ip, &mac_bytes, 2, &setup.dhcp); // OFFER
                logging::log_info(&format!("DHCP OFFER sent to {} ({})", ip, mac_str));
            }
        } else if msg_type == 3 {
            // REQUEST -> Client is requesting an IP. Prefer requested_ip (option 50) if present and available.
            let mut assigned: Option<String> = None;
            if let Some(req_ip) = requested_ip.clone() {
                // If requested IP is not leased (or leased to same MAC), honor it
                let leases = list_leases();
                if !leases.iter().any(|l| l.ip == req_ip && l.mac != mac_str) {
                    // Persist this requested IP
                    match allocate_ip_for_requested(&mac_str, &hostname, &setup.dhcp, &req_ip) {
                        Some(ip) => assigned = Some(ip),
                        None => {}
                    }
                }
            }

            if assigned.is_none() {
                // Fallback: allocate next available IP and persist
                assigned = allocate_ip(&mac_str, &hostname, &setup.dhcp);
            }

            if let Some(ip) = assigned {
                send_dhcp_reply(socket, packet, xid, &ip, &mac_bytes, 5, &setup.dhcp); // ACK
                let msg = format!("New Lease: {} ({}) - {}", hostname, ip, mac_str);
                logging::log_info(&msg);
                if let Some(h) = app {
                    notify::send_notification(h, "DHCP Lease", &msg, "info");
                }
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
        
        // Ensure we don't allocate the Gateway IP or the Server Bind Address
        if current_ip == config.gateway || current_ip == config.bind_address {
            // Increment IP and continue
            for i in (0..4).rev() {
                if current[i] < 255 {
                    current[i] += 1;
                    break;
                } else {
                    current[i] = 0;
                }
            }
            continue;
        }

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
            logging::log_debug(&format!("Allocated IP {} to MAC {} (hostname={})", current_ip, mac, hostname));
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

/// Find a free IP in the pool without persisting a lease. If the MAC already has a lease, return it.
fn find_free_ip(config: &config::DhcpConfig, mac_opt: Option<&str>) -> Option<String> {
    let leases = list_leases();

    // If MAC already has a lease, return that IP
    if let Some(mac) = mac_opt {
        if let Some(l) = leases.iter().find(|l| l.mac == mac) {
            return Some(l.ip.clone());
        }
    }

    let start_parts: Vec<u8> = config.range_start.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let end_parts: Vec<u8> = config.range_end.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    if start_parts.len() != 4 || end_parts.len() != 4 { return None; }

    let mut current = start_parts.clone();
    while current <= end_parts {
        let current_ip = format!("{}.{}.{}.{}", current[0], current[1], current[2], current[3]);
        if current_ip == config.gateway || current_ip == config.bind_address {
            for i in (0..4).rev() { if current[i] < 255 { current[i] += 1; break; } else { current[i] = 0; } }
            continue;
        }
        if !leases.iter().any(|l| l.ip == current_ip) {
            return Some(current_ip);
        }
        for i in (0..4).rev() { if current[i] < 255 { current[i] += 1; break; } else { current[i] = 0; } }
    }
    None
}

/// Allocate a specific requested IP for a MAC and persist the lease. Returns the IP if successful.
fn allocate_ip_for_requested(mac: &str, hostname: &str, config: &config::DhcpConfig, requested_ip: &str) -> Option<String> {
    let mut leases = list_leases();

    // If requested IP is already used by another MAC, fail
    if leases.iter().any(|l| l.ip == requested_ip && l.mac != mac) {
        return None;
    }

    // Remove any existing lease entries for this IP or MAC
    leases.retain(|l| !(l.ip == requested_ip) && !(l.mac == mac));

    let expires = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + config.lease_time;
    leases.push(Lease {
        ip: requested_ip.to_string(),
        mac: mac.to_string(),
        hostname: hostname.to_string(),
        expires_at: expires.to_string(),
        static_lease: false,
    });

    if let Ok(serialized) = serde_json::to_string_pretty(&leases) {
        let _ = post::write_file(&get_leases_file(), &serialized);
    }
    Some(requested_ip.to_string())
}

fn send_dhcp_reply(socket: &UdpSocket, _req: &[u8], xid: &[u8], yiaddr: &str, chaddr: &[u8], msg_type: u8, config: &config::DhcpConfig) {
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
    match socket.send_to(&packet[..i+1], "255.255.255.255:68") {
        Ok(n) => logging::log_debug(&format!("DHCP reply (type {}) sent ({} bytes)", msg_type, n)),
        Err(e) => logging::log_error(&format!("Failed sending DHCP reply: {}", e)),
    }
}
