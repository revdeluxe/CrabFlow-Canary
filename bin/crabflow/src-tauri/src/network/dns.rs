// src-tauri/src/dns.rs

use serde::{Serialize, Deserialize};
use crate::sysmodules::{fetch, post, logging, config};
use dotenv::var;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

static DNS_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DnsRecord {
    pub name: String,
    pub rtype: String, // "A","AAAA","CNAME","MX"
    pub value: String,
    pub ttl: u32,
}

#[derive(Deserialize)]
pub struct DnsRecordInput {
    pub name: String,
    pub rtype: String,
    pub value: String,
    pub ttl: u32,
}

fn get_dns_file() -> String {
    var("CRABFLOW_DNS_CONFIG").unwrap_or_else(|_| "dns.json".to_string())
}

/// List all DNS records
pub fn list_records() -> Vec<DnsRecord> {
    match fetch::read_file(&get_dns_file()) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => vec![],
    }
}

/// Add a DNS record
pub fn add_record(input: DnsRecordInput) -> Result<(), String> {
    let mut records = list_records();
    let name_for_log = input.name.clone();
    records.push(DnsRecord {
        name: input.name,
        rtype: input.rtype,
        value: input.value,
        ttl: input.ttl,
    });

    let serialized = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
    post::write_file(&get_dns_file(), &serialized)?;
    logging::log_event("system".into(), "add_record".into(), name_for_log);
    Ok(())
}

// --- DNS Server Implementation ---

pub fn stop_dns_server() {
    if DNS_RUNNING.load(Ordering::Relaxed) {
        logging::log_info("Stopping DNS Server...");
        DNS_RUNNING.store(false, Ordering::Relaxed);
    }
}

pub fn start_dns_server() {
    if DNS_RUNNING.load(Ordering::Relaxed) {
        logging::log_info("DNS Server is already running.");
        return;
    }

    DNS_RUNNING.store(true, Ordering::Relaxed);

    thread::spawn(|| {
        let socket = match UdpSocket::bind("0.0.0.0:53") {
            Ok(s) => s,
            Err(e) => {
                logging::log_error(&format!("Failed to bind DNS server to port 53: {}", e));
                DNS_RUNNING.store(false, Ordering::Relaxed);
                return;
            }
        };

        socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap_or_else(|e| logging::log_error(&format!("Failed to set read timeout: {}", e)));
        logging::log_info("DNS Server started on port 53");

        let mut buf = [0u8; 512];
        while DNS_RUNNING.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let query = &buf[..amt];
                    // Simple check: Is it a standard query?
                    // ID (2), Flags (2). Flags 0x0100 (Standard Query)
                    // We just respond to everything with a basic handler for now.
                    
                    if let Some(response) = handle_dns_query(query) {
                        let _ = socket.send_to(&response, src);
                    }
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock && e.kind() != std::io::ErrorKind::TimedOut {
                        logging::log_error(&format!("Error receiving DNS packet: {}", e));
                    }
                }
            }
        }
        logging::log_info("DNS Server stopped.");
    });
}

fn handle_dns_query(query: &[u8]) -> Option<Vec<u8>> {
    if query.len() < 12 { return None; }

    // Parse Header
    let id = &query[0..2];
    let flags = &query[2..4];
    let qdcount = u16::from_be_bytes([query[4], query[5]]);
    
    // Only handle single question for simplicity
    if qdcount != 1 { return None; }

    // Parse Question Name
    let mut pos = 12;
    let mut domain_parts = Vec::new();
    
    loop {
        if pos >= query.len() { return None; }
        let len = query[pos] as usize;
        if len == 0 {
            pos += 1;
            break;
        }
        pos += 1;
        if pos + len > query.len() { return None; }
        if let Ok(part) = std::str::from_utf8(&query[pos..pos+len]) {
            domain_parts.push(part);
        }
        pos += len;
    }
    
    let domain = domain_parts.join(".");
    
    // Type and Class
    if pos + 4 > query.len() { return None; }
    let qtype = u16::from_be_bytes([query[pos], query[pos+1]]);
    let qclass = u16::from_be_bytes([query[pos+2], query[pos+3]]);
    
    // Only handle A records (Type 1) and IN Class (1)
    if qtype != 1 || qclass != 1 { 
        // TODO: Forward or ignore? For now, ignore.
        return None; 
    }

    // Lookup Logic
    // 1. Check local records
    // 2. Check Captive Portal status (Mocked for now: if domain is "captive.portal", return local IP)
    // 3. Forward (Not implemented yet, just return NXDOMAIN or specific IP)

    let records = list_records();
    let answer_ip = records.iter().find(|r| r.name == domain && r.rtype == "A").map(|r| r.value.clone());
    
    // Mock Captive Portal Redirection
    // If we don't know the domain, and we are in "Strict" mode, we might return our own IP.
    // For now, let's just return a dummy IP if found, or nothing (timeout) if not found.
    // Or better: Forward to 8.8.8.8?
    // Since we can't easily forward without a full client, let's just serve local records.
    
    if let Some(ip_str) = answer_ip {
        return Some(build_dns_response(id, query, &ip_str));
    }
    
    // Fallback: If domain is "connectivitycheck.gstatic.com" or similar, return our IP for portal testing
    if domain.contains("connectivitycheck") || domain.contains("msftconnecttest") {
         // Return Gateway IP (Hardcoded for now, should fetch from config)
         return Some(build_dns_response(id, query, "10.0.0.1"));
    }

    None
}

fn build_dns_response(id: &[u8], query: &[u8], ip_str: &str) -> Vec<u8> {
    let mut response = Vec::new();
    
    // Header
    response.extend_from_slice(id);
    response.extend_from_slice(&[0x81, 0x80]); // Standard Response, No Error
    response.extend_from_slice(&[0x00, 0x01]); // QDCOUNT = 1
    response.extend_from_slice(&[0x00, 0x01]); // ANCOUNT = 1
    response.extend_from_slice(&[0x00, 0x00]); // NSCOUNT = 0
    response.extend_from_slice(&[0x00, 0x00]); // ARCOUNT = 0
    
    // Question Section (Copy from query)
    // We need the length of the question section.
    // It's 12 (header) + name length + 4 (type/class)
    // We can just find the end of the name again or pass it.
    // Hack: Just copy everything after header from query?
    // The query passed to this function includes the header.
    // We need to find where the question ends.
    
    let mut pos = 12;
    loop {
        let len = query[pos] as usize;
        if len == 0 { pos += 1; break; }
        pos += 1 + len;
    }
    pos += 4; // Type + Class
    
    response.extend_from_slice(&query[12..pos]);
    
    // Answer Section
    // Name: Pointer to offset 12 (0xC00C)
    response.extend_from_slice(&[0xC0, 0x0C]);
    
    // Type: A (1)
    response.extend_from_slice(&[0x00, 0x01]);
    // Class: IN (1)
    response.extend_from_slice(&[0x00, 0x01]);
    // TTL: 60 seconds
    response.extend_from_slice(&[0x00, 0x00, 0x00, 0x3C]);
    // RDLength: 4
    response.extend_from_slice(&[0x00, 0x04]);
    // RData: IP Address
    let parts: Vec<u8> = ip_str.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    response.extend_from_slice(&parts);
    
    response
}

/// Remove a DNS record
pub fn remove_record(name: String, rtype: String) -> Result<(), String> {
    let mut records = list_records();
    records.retain(|r| !(r.name == name && r.rtype == rtype));

    let serialized = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
    post::write_file(&get_dns_file(), &serialized)?;
    logging::log_event("system".into(), "remove_record".into(), name);
    Ok(())
}
