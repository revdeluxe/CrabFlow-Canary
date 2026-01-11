// src-tauri/src/dns.rs

use serde::{Serialize, Deserialize};
use crate::sysmodules::{fetch, post, logging, config, notify, paths};
use crate::network::{dhcp, acl};
use tauri::AppHandle;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, RwLock};
use lazy_static::lazy_static;
use std::collections::{HashSet, VecDeque};

static DNS_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DnsQueryLog {
    pub timestamp: u64,
    pub client_ip: String,
    pub domain: String,
    pub query_type: String,
    pub status: String, // "Allowed", "Blocked"
}

lazy_static! {
    static ref QUERY_LOG: Mutex<VecDeque<DnsQueryLog>> = Mutex::new(VecDeque::new());
    static ref BLACKLIST_CACHE: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
    static ref RECORDS_CACHE: RwLock<Vec<DnsRecord>> = RwLock::new(Vec::new());
    static ref AUTHENTICATED_IPS: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
    // Default upstream, can be made configurable later
    static ref UPSTREAM_DNS: RwLock<Vec<String>> = RwLock::new(vec!["1.1.1.1:53".to_string(), "8.8.8.8:53".to_string()]);
    static ref UPSTREAM_INTERFACE: RwLock<String> = RwLock::new("0.0.0.0".to_string());
}

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

pub fn authorize_ip(ip: String) {
    let mut cache = AUTHENTICATED_IPS.write().unwrap();
    cache.insert(ip.clone());
    logging::log_info(&format!("Authorized IP for Internet: {}", ip));
}

fn get_dns_file() -> String {
    paths::get_config_path("dns.json").to_string_lossy().to_string()
}

/// Initialize caches from disk
pub fn init_dns_caches() {
    // Load Blacklist
    match fetch::read_file(&get_blacklist_file()) {
        Ok(data) => {
            let list: Vec<String> = serde_json::from_str(&data).unwrap_or_default();
            let mut cache = BLACKLIST_CACHE.write().unwrap();
            *cache = list.into_iter().collect();
        },
        Err(_) => {}
    }

    // Load Records
    match fetch::read_file(&get_dns_file()) {
        Ok(data) => {
            let list: Vec<DnsRecord> = serde_json::from_str(&data).unwrap_or_default();
            let mut cache = RECORDS_CACHE.write().unwrap();
            *cache = list;
        },
        Err(_) => {}
    }
    logging::log_info("DNS caches initialized");
}

pub fn set_upstream_interface(ip: String) {
    let mut iface = UPSTREAM_INTERFACE.write().unwrap();
    *iface = ip;
}

/// List all DNS records
pub fn list_records() -> Vec<DnsRecord> {
    RECORDS_CACHE.read().unwrap().clone()
}

/// Add a DNS record
pub fn add_record(input: DnsRecordInput) -> Result<(), String> {
    let mut records = list_records(); // Get current copy
    let name_for_log = input.name.clone();
    records.push(DnsRecord {
        name: input.name,
        rtype: input.rtype,
        value: input.value,
        ttl: input.ttl,
    });

    // Update Cache
    {
        let mut cache = RECORDS_CACHE.write().unwrap();
        *cache = records.clone();
    }

    let serialized = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
    post::write_file(&get_dns_file(), &serialized)?;
    logging::log_event("system".into(), "add_record".into(), name_for_log);
    Ok(())
}

#[tauri::command]
pub fn get_query_logs(limit: usize) -> Vec<DnsQueryLog> {
    let logs = QUERY_LOG.lock().unwrap();
    logs.iter().rev().take(limit).cloned().collect()
}

fn log_query(client_ip: String, domain: String, query_type: String, status: String) {
    let mut logs = QUERY_LOG.lock().unwrap();
    logs.push_back(DnsQueryLog {
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        client_ip,
        domain,
        query_type,
        status,
    });
    // Keep log size manageable
    if logs.len() > 10000 {
        logs.pop_front();
    }
}

// --- DNS Server Implementation ---

pub fn stop_dns_server() {
    if DNS_RUNNING.load(Ordering::Relaxed) {
        logging::log_info("Stopping DNS Server...");
        DNS_RUNNING.store(false, Ordering::Relaxed);
    }
}

pub fn is_server_running() -> bool {
    DNS_RUNNING.load(Ordering::Relaxed)
}

pub fn get_query_count() -> usize {
    let logs = QUERY_LOG.lock().unwrap();
    logs.len()
}

pub fn start_dns_server(app_handle: Option<AppHandle>) {
    if DNS_RUNNING.load(Ordering::Relaxed) {
        logging::log_info("DNS Server is already running.");
        return;
    }

    // Initialize caches before starting
    init_dns_caches();
    
    // Ensure Gateway FQDN for Captive Portal
    match fetch::fetch_setup() {
        Ok(cfg) => {
             let fqdn = if cfg.advanced.captive_portal_domain.is_empty() { 
                 "crabflow.login".to_string() 
             } else { 
                 cfg.advanced.captive_portal_domain 
             };
             
             // Inject Record for Gateway IP
             // Assuming Gateway IP matches the DHCP server IP
             let gateway_ip = cfg.dhcp.gateway;
             logging::log_info(&format!("Injecting DNS A record: {} -> {}", fqdn, gateway_ip));
             
             let _ = add_record(DnsRecordInput {
                 name: fqdn,
                 rtype: "A".to_string(),
                 value: gateway_ip,
                 ttl: 300,
             });
        },
        Err(e) => {
            logging::log_warn(&format!("Could not load setup for DNS FQDN injection: {}", e));
        }
    }

    DNS_RUNNING.store(true, Ordering::Relaxed);
    let app = app_handle.clone();

    thread::spawn(move || {
        let socket = match UdpSocket::bind("0.0.0.0:53") {
            Ok(s) => s,
            Err(e) => {
                let err_msg = format!("Failed to bind DNS server to port 53: {}", e);
                logging::log_error(&err_msg);
                if let Some(h) = &app {
                    notify::send_notification(h, "DNS Server Error", &err_msg, "error");
                }
                DNS_RUNNING.store(false, Ordering::Relaxed);
                return;
            }
        };

        socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap_or_else(|e| logging::log_error(&format!("Failed to set read timeout: {}", e)));
        logging::log_info("DNS Server started on port 53");
        if let Some(h) = &app {
            notify::send_notification(h, "DNS Server Started", "Listening on port 53", "success");
        }

        let mut buf = [0u8; 512];
        // Cache config to avoid reading disk on every packet
        let mut last_config_check = std::time::Instant::now();
        let mut allow_non_dhcp = true;

        if let Ok(cfg) = config::load_setup_config() {
             allow_non_dhcp = cfg.dns.allow_non_dhcp_clients;
        }

        while DNS_RUNNING.load(Ordering::Relaxed) {
            // Update config cache every 5 seconds
            if last_config_check.elapsed().as_secs() > 5 {
                if let Ok(cfg) = config::load_setup_config() {
                     allow_non_dhcp = cfg.dns.allow_non_dhcp_clients;
                }
                last_config_check = std::time::Instant::now();
            }

            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let query = &buf[..amt];
                    let src_ip = src.ip().to_string();

                    // ACL Check
                    if !allow_non_dhcp {
                        // We check leases only if restricted. 
                        // is_ip_leased still reads disk, but filtering is usually for security so correctness > speed?
                        // Or we could cache leases? For now, we accept the overhead.
                        if !dhcp::is_ip_leased(&src_ip) {
                            logging::log_debug(&format!("Blocked DNS query from non-DHCP client: {}", src_ip));
                            continue; // Drop packet
                        }
                    }

                    // Simple check: Is it a standard query?
                    // ID (2), Flags (2). Flags 0x0100 (Standard Query)
                    // We just respond to everything with a basic handler for now.
                    
                    if let Some((response, domain, status)) = handle_dns_query(query, &src_ip) {
                        if status == "Blocked" {
                             if let Some(h) = &app {
                                notify::send_notification(h, "DNS Blocked", &format!("Access to {} blocked from {}", domain, src_ip), "warning");
                             }
                        }
                        log_query(src_ip, domain, "A".to_string(), status);
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

fn forward_dns_query(query: &[u8]) -> Option<Vec<u8>> {
    let upstreams = UPSTREAM_DNS.read().unwrap();
    let bind_ip = UPSTREAM_INTERFACE.read().unwrap();
    let bind_addr = format!("{}:0", bind_ip);

    for upstream in upstreams.iter() {
        match UdpSocket::bind(&bind_addr) {
            Ok(socket) => {
                socket.set_read_timeout(Some(Duration::from_secs(2))).ok();
                if socket.send_to(query, upstream).is_ok() {
                    let mut buf = [0u8; 512];
                    if let Ok((amt, _)) = socket.recv_from(&mut buf) {
                        return Some(buf[..amt].to_vec());
                    }
                }
            },
            Err(e) => {
                logging::log_error(&format!("Failed to bind forwarder to {}: {}", bind_addr, e));
                continue;
            },
        }
    }
    None
}

fn handle_dns_query(query: &[u8], src_ip: &str) -> Option<(Vec<u8>, String, String)> {
    if query.len() < 12 { return None; }

    // Parse Header
    let id = &query[0..2];
    let _flags = &query[2..4];
    let qdcount = u16::from_be_bytes([query[4], query[5]]);
    
    // Only handle single question for simplicity
    if qdcount != 1 { return None; }

    // Parse Question Section to get Name
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
    let domain_name = domain_parts.join(".");

    // Check Auth Status (Captive Portal)
    let is_auth = {
        let cache = AUTHENTICATED_IPS.read().unwrap();
        cache.contains(src_ip)
    };

    // Check if captive portal is enabled via ACL config
    let captive_portal_enabled = acl::is_captive_portal_enabled();
    
    // Check if this is a captive portal detection domain (triggers "Sign in to network" prompt)
    let is_detection_domain = acl::is_detection_domain(&domain_name);
    
    // Check if domain should be allowed before authentication
    let is_allowed_before_auth = acl::is_allowed_before_auth(&domain_name);

    // The Hijack Logic - Only if captive portal is enabled
    if captive_portal_enabled && !is_auth && !is_allowed_before_auth {
        // Retrieve valid gateway/interface IP (The one user can reach to login)
        let gateway_ip = UPSTREAM_INTERFACE.read().unwrap().clone();
        
        // For detection domains, we need special handling to trigger the captive portal prompt
        if is_detection_domain {
            // Return the gateway IP - the HTTP server will handle the captive portal response
            let spoofed_response = build_dns_response(id, query, &gateway_ip);
            logging::log_debug(&format!("Captive Portal Detection: {} -> {} (from {})", domain_name, gateway_ip, src_ip));
            return Some((spoofed_response, domain_name, "CaptiveDetect".to_string()));
        }
        
        // If the user asks for "portal.crabflow.local", give them the real IP (which is gateway_ip anyway usually).
        // If they ask for ANYTHING else (google.com), give them the Gateway IP too.
        
        // Construct spoofed response
        let spoofed_response = build_dns_response(id, query, &gateway_ip);

        // We can optionally check if domain_name == "portal.crabflow.local" to mark as 'Allowed' instead of 'Redirected'
        // but broadly we serve the same content.
        let status = if domain_name.contains("crabflow") { "Portal".to_string() } else { "Redirected".to_string() };

        return Some((spoofed_response, domain_name, status));
    }

    // Skip QTYPE and QCLASS
    pos += 4;

    // Check Blacklist (Cache)
    let is_blocked = {
        let cache = BLACKLIST_CACHE.read().unwrap();
        cache.contains(&domain_name)
    };

    // Check records (Cache)
    let records = RECORDS_CACHE.read().unwrap();
    let answer = records.iter().find(|r| r.name == domain_name && r.rtype == "A");

    // If not blocked and not local, forward
    if !is_blocked && answer.is_none() {
        if let Some(response) = forward_dns_query(query) {
            return Some((response, domain_name, "Forwarded".to_string()));
        }
        // If forwarding fails, fall through to NXDOMAIN
    }

    let mut response = Vec::new();
    response.extend_from_slice(id); // ID
    
    if is_blocked {
        response.extend_from_slice(&[0x81, 0x80]); // Standard Response, No Error
    } else if let Some(_rec) = answer {
        // Standard Response, No Error
        response.extend_from_slice(&[0x81, 0x80]); // Flags: QR=1, AA=0, TC=0, RD=1, RA=1, Z=0, RCODE=0
    } else {
        // NXDOMAIN
        response.extend_from_slice(&[0x81, 0x83]); // Flags: RCODE=3 (NXDOMAIN)
    }

    response.extend_from_slice(&query[4..6]); // QDCOUNT
    if is_blocked || answer.is_some() {
        response.extend_from_slice(&[0x00, 0x01]); // ANCOUNT = 1
    } else {
        response.extend_from_slice(&[0x00, 0x00]); // ANCOUNT = 0
    }
    response.extend_from_slice(&[0x00, 0x00]); // NSCOUNT
    response.extend_from_slice(&[0x00, 0x00]); // ARCOUNT

    // Question Section (Copy from query)
    // We need to copy up to the end of the question section which we calculated as `pos`
    response.extend_from_slice(&query[12..pos]);

    // Answer Section
    if is_blocked {
        // Name pointer (0xC00C points to start of question name)
        response.extend_from_slice(&[0xc0, 0x0c]);
        // Type A (1)
        response.extend_from_slice(&[0x00, 0x01]);
        // Class IN (1)
        response.extend_from_slice(&[0x00, 0x01]);
        // TTL (0)
        response.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        // RDLENGTH (4 for IPv4)
        response.extend_from_slice(&[0x00, 0x04]);
        // RDATA (0.0.0.0)
        response.extend_from_slice(&[0,0,0,0]);
    } else if let Some(rec) = answer {
        // Name pointer (0xC00C points to start of question name)
        response.extend_from_slice(&[0xc0, 0x0c]);
        // Type A (1)
        response.extend_from_slice(&[0x00, 0x01]);
        // Class IN (1)
        response.extend_from_slice(&[0x00, 0x01]);
        // TTL
        response.extend_from_slice(&rec.ttl.to_be_bytes());
        // RDLENGTH (4 for IPv4)
        response.extend_from_slice(&[0x00, 0x04]);
        // RDATA (IP Address)
        if let Ok(ip) = rec.value.parse::<std::net::Ipv4Addr>() {
            response.extend_from_slice(&ip.octets());
        } else {
            // Fallback or error handling
            response.extend_from_slice(&[0,0,0,0]);
        }
    }

    let status = if is_blocked { "Blocked".to_string() } else { "Allowed".to_string() };
    Some((response, domain_name, status))
}

fn get_blacklist_file() -> String {
    paths::get_config_path("blacklist.json").to_string_lossy().to_string()
}

pub fn list_blacklist() -> Vec<String> {
    BLACKLIST_CACHE.read().unwrap().iter().cloned().collect()
}

#[tauri::command]
pub fn get_blacklist() -> Vec<String> {
    list_blacklist()
}

#[tauri::command]
pub fn block_domain(domain: String) -> Result<(), String> {
    let mut list = list_blacklist();
    if !list.contains(&domain) {
        list.push(domain.clone());
        
        // Update Cache
        {
            let mut cache = BLACKLIST_CACHE.write().unwrap();
            cache.insert(domain.clone());
        }

        let serialized = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
        post::write_file(&get_blacklist_file(), &serialized)?;
        logging::log_event("system".into(), "block_domain".into(), domain);
    }
    Ok(())
}

#[tauri::command]
pub fn unblock_domain(domain: String) -> Result<(), String> {
    let mut list = list_blacklist();
    if let Some(pos) = list.iter().position(|x| *x == domain) {
        list.remove(pos);

        // Update Cache
        {
            let mut cache = BLACKLIST_CACHE.write().unwrap();
            cache.remove(&domain);
        }

        let serialized = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
        post::write_file(&get_blacklist_file(), &serialized)?;
        logging::log_event("system".into(), "unblock_domain".into(), domain);
    }
    Ok(())
}

#[tauri::command]
pub fn import_blacklist(domains: Vec<String>) -> Result<usize, String> {
    let mut list = list_blacklist();
    let mut count = 0;
    let mut cache = BLACKLIST_CACHE.write().unwrap();

    for domain in domains {
        if !cache.contains(&domain) {
            list.push(domain.clone());
            cache.insert(domain);
            count += 1;
        }
    }
    if count > 0 {
        let serialized = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
        post::write_file(&get_blacklist_file(), &serialized)?;
        logging::log_event("system".into(), "import_blacklist".into(), format!("Imported {} domains", count));
    }
    Ok(count)
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

/// Update a DNS record
pub fn update_record(old_name: String, old_rtype: String, input: DnsRecordInput) -> Result<(), String> {
    let mut records = list_records();
    if let Some(index) = records.iter().position(|r| r.name == old_name && r.rtype == old_rtype) {
        let name_for_log = input.name.clone();
        records[index] = DnsRecord {
            name: input.name,
            rtype: input.rtype,
            value: input.value,
            ttl: input.ttl,
        };
        
        {
            let mut cache = RECORDS_CACHE.write().unwrap();
            *cache = records.clone();
        }

        let serialized = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
        post::write_file(&get_dns_file(), &serialized)?;
        logging::log_event("system".into(), "update_record".into(), name_for_log);
        Ok(())
    } else {
        Err("Record not found".to_string())
    }
}

/// Remove a DNS record
pub fn remove_record(name: String, rtype: String) -> Result<(), String> {
    let mut records = list_records();
    records.retain(|r| !(r.name == name && r.rtype == rtype));

    // Update Cache
    {
        let mut cache = RECORDS_CACHE.write().unwrap();
        *cache = records.clone();
    }

    let serialized = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
    post::write_file(&get_dns_file(), &serialized)?;
    logging::log_event("system".into(), "remove_record".into(), name);
    Ok(())
}
