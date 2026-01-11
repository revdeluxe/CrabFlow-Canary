// src-tauri/src/network/acl.rs
// Access Control List management for routes, forwarding, captive portal, and dataflow

use serde::{Deserialize, Serialize};
use crate::sysmodules::{paths, logging};
use std::fs;
use std::sync::RwLock;
use lazy_static::lazy_static;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AclConfig {
    #[serde(default)]
    pub captive_portal: CaptivePortalConfig,
    #[serde(default)]
    pub routes: Vec<RouteRule>,
    #[serde(default)]
    pub forwarding: ForwardingConfig,
    #[serde(default)]
    pub dataflow: DataflowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptivePortalConfig {
    pub enabled: bool,
    pub redirect_url: String,
    pub auth_required: bool,
    pub session_timeout: u64,
    pub allowed_domains: Vec<String>,
    pub detection_domains: Vec<String>,
}

impl Default for CaptivePortalConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            redirect_url: "http://portal.crabflow.local".to_string(),
            auth_required: true,
            session_timeout: 3600,
            allowed_domains: vec![],
            detection_domains: vec![
                "www.msftconnecttest.com".to_string(),
                "msftconnecttest.com".to_string(),
                "captive.apple.com".to_string(),
                "www.apple.com".to_string(),
                "connectivitycheck.gstatic.com".to_string(),
                "clients3.google.com".to_string(),
                "connectivitycheck.android.com".to_string(),
                "www.gstatic.com".to_string(),
                "play.googleapis.com".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRule {
    pub name: String,
    pub source: String,       // IP/CIDR or "*"
    pub destination: String,  // IP/CIDR or "*"
    pub port: String,         // Port number, range, or "*"
    pub protocol: String,     // "any", "tcp", "udp", "icmp"
    pub action: String,       // "allow", "deny", "forward"
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardingConfig {
    pub enabled: bool,
    pub nat_enabled: bool,
    pub uplink: String,       // Interface name
    pub downlink: String,     // Interface name
    pub rules: Vec<ForwardingRule>,
}

impl Default for ForwardingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            nat_enabled: true,
            uplink: String::new(),
            downlink: String::new(),
            rules: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardingRule {
    pub name: String,
    #[serde(rename = "sourceNetwork")]
    pub source_network: String,
    #[serde(rename = "targetInterface")]
    pub target_interface: String,
    pub masquerade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataflowConfig {
    pub bandwidth: BandwidthLimits,
    pub qos_enabled: bool,
    pub qos_rules: Vec<QosRule>,
    #[serde(default)]
    pub group_limits: Vec<GroupBandwidthLimit>,
}

impl Default for DataflowConfig {
    fn default() -> Self {
        Self {
            bandwidth: BandwidthLimits::default(),
            qos_enabled: false,
            qos_rules: vec![],
            group_limits: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthLimits {
    #[serde(rename = "globalUpload")]
    pub global_upload: u64,
    #[serde(rename = "globalDownload")]
    pub global_download: u64,
    #[serde(rename = "perClientUpload")]
    pub per_client_upload: u64,
    #[serde(rename = "perClientDownload")]
    pub per_client_download: u64,
    pub enabled: bool,
}

impl Default for BandwidthLimits {
    fn default() -> Self {
        Self {
            global_upload: 0,
            global_download: 0,
            per_client_upload: 0,
            per_client_download: 0,
            enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupBandwidthLimit {
    pub group_name: String,
    pub upload_limit: u64,    // Mbps, 0 = unlimited
    pub download_limit: u64,  // Mbps, 0 = unlimited
    pub priority: u32,        // 1-10, higher = more priority
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QosRule {
    pub name: String,
    pub priority: u32,
    pub traffic_type: String,
    pub bandwidth_percent: u32,
}

// ============================================================================
// Global Cache
// ============================================================================

lazy_static! {
    static ref ACL_CONFIG: RwLock<AclConfig> = RwLock::new(AclConfig::default());
}

fn get_acl_config_path() -> std::path::PathBuf {
    paths::get_config_path("acl_config.json")
}

// ============================================================================
// Config Management
// ============================================================================

pub fn init_acl() {
    match load_acl_config_from_disk() {
        Ok(config) => {
            let mut cache = ACL_CONFIG.write().unwrap();
            *cache = config;
            logging::log_info("ACL configuration loaded");
        }
        Err(e) => {
            logging::log_warn(&format!("Could not load ACL config, using defaults: {}", e));
        }
    }
}

fn load_acl_config_from_disk() -> Result<AclConfig, String> {
    let path = get_acl_config_path();
    if !path.exists() {
        return Ok(AclConfig::default());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

fn save_acl_config_to_disk(config: &AclConfig) -> Result<(), String> {
    let path = get_acl_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// Tauri Commands
// ============================================================================

#[tauri::command]
pub fn get_acl_config() -> Result<AclConfig, String> {
    let cache = ACL_CONFIG.read().map_err(|e| e.to_string())?;
    Ok(cache.clone())
}

#[tauri::command]
pub fn save_acl_config(config: AclConfig) -> Result<(), String> {
    // Save to disk
    save_acl_config_to_disk(&config)?;
    
    // Update cache
    let mut cache = ACL_CONFIG.write().map_err(|e| e.to_string())?;
    *cache = config;
    
    logging::log_info("ACL configuration saved");
    Ok(())
}

// ============================================================================
// Helper Functions for DNS/DHCP integration
// ============================================================================

/// Check if captive portal is enabled
pub fn is_captive_portal_enabled() -> bool {
    if let Ok(cache) = ACL_CONFIG.read() {
        cache.captive_portal.enabled
    } else {
        false
    }
}

/// Get captive portal detection domains
pub fn get_detection_domains() -> Vec<String> {
    if let Ok(cache) = ACL_CONFIG.read() {
        cache.captive_portal.detection_domains.clone()
    } else {
        CaptivePortalConfig::default().detection_domains
    }
}

/// Get domains allowed before authentication
pub fn get_allowed_domains() -> Vec<String> {
    if let Ok(cache) = ACL_CONFIG.read() {
        cache.captive_portal.allowed_domains.clone()
    } else {
        vec![]
    }
}

/// Check if a domain is a captive portal detection domain
pub fn is_detection_domain(domain: &str) -> bool {
    let detection_domains = get_detection_domains();
    let domain_lower = domain.to_lowercase();
    detection_domains.iter().any(|d| {
        domain_lower == d.to_lowercase() || domain_lower.ends_with(&format!(".{}", d.to_lowercase()))
    })
}

/// Check if a domain should be allowed before authentication
pub fn is_allowed_before_auth(domain: &str) -> bool {
    let allowed = get_allowed_domains();
    let domain_lower = domain.to_lowercase();
    allowed.iter().any(|d| {
        domain_lower == d.to_lowercase() || domain_lower.ends_with(&format!(".{}", d.to_lowercase()))
    })
}

/// Get session timeout
pub fn get_session_timeout() -> u64 {
    if let Ok(cache) = ACL_CONFIG.read() {
        cache.captive_portal.session_timeout
    } else {
        3600
    }
}

/// Check if forwarding is enabled
pub fn is_forwarding_enabled() -> bool {
    if let Ok(cache) = ACL_CONFIG.read() {
        cache.forwarding.enabled
    } else {
        false
    }
}

/// Get forwarding interfaces
pub fn get_forwarding_interfaces() -> (String, String) {
    if let Ok(cache) = ACL_CONFIG.read() {
        (cache.forwarding.uplink.clone(), cache.forwarding.downlink.clone())
    } else {
        (String::new(), String::new())
    }
}

/// Evaluate a route rule against traffic
pub fn evaluate_route(src: &str, dst: &str, port: u16, protocol: &str) -> Option<String> {
    if let Ok(cache) = ACL_CONFIG.read() {
        let mut rules = cache.routes.clone();
        rules.sort_by(|a, b| a.priority.cmp(&b.priority));
        
        for rule in rules {
            if matches_rule(&rule, src, dst, port, protocol) {
                return Some(rule.action.clone());
            }
        }
    }
    // Default action: allow
    Some("allow".to_string())
}

fn matches_rule(rule: &RouteRule, src: &str, dst: &str, port: u16, protocol: &str) -> bool {
    // Check protocol
    if rule.protocol != "any" && rule.protocol.to_lowercase() != protocol.to_lowercase() {
        return false;
    }
    
    // Check source
    if rule.source != "*" && !matches_ip_pattern(&rule.source, src) {
        return false;
    }
    
    // Check destination
    if rule.destination != "*" && !matches_ip_pattern(&rule.destination, dst) {
        return false;
    }
    
    // Check port
    if rule.port != "*" && !matches_port_pattern(&rule.port, port) {
        return false;
    }
    
    true
}

fn matches_ip_pattern(pattern: &str, ip: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    
    // Simple CIDR matching (basic implementation)
    if pattern.contains('/') {
        // CIDR notation
        let parts: Vec<&str> = pattern.split('/').collect();
        if parts.len() == 2 {
            let net_ip = parts[0];
            if let Ok(prefix_len) = parts[1].parse::<u32>() {
                return ip_in_cidr(ip, net_ip, prefix_len);
            }
        }
    }
    
    // Exact match
    pattern == ip
}

fn ip_in_cidr(ip: &str, network: &str, prefix_len: u32) -> bool {
    let ip_parts: Vec<u32> = ip.split('.').filter_map(|s| s.parse().ok()).collect();
    let net_parts: Vec<u32> = network.split('.').filter_map(|s| s.parse().ok()).collect();
    
    if ip_parts.len() != 4 || net_parts.len() != 4 {
        return false;
    }
    
    let ip_num = (ip_parts[0] << 24) | (ip_parts[1] << 16) | (ip_parts[2] << 8) | ip_parts[3];
    let net_num = (net_parts[0] << 24) | (net_parts[1] << 16) | (net_parts[2] << 8) | net_parts[3];
    
    let mask = if prefix_len == 0 { 0 } else { !0u32 << (32 - prefix_len) };
    
    (ip_num & mask) == (net_num & mask)
}

fn matches_port_pattern(pattern: &str, port: u16) -> bool {
    if pattern == "*" {
        return true;
    }
    
    // Handle comma-separated ports
    for part in pattern.split(',') {
        let part = part.trim();
        
        // Handle range
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                if let (Ok(start), Ok(end)) = (range[0].parse::<u16>(), range[1].parse::<u16>()) {
                    if port >= start && port <= end {
                        return true;
                    }
                }
            }
        } else if let Ok(p) = part.parse::<u16>() {
            if p == port {
                return true;
            }
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_in_cidr() {
        assert!(ip_in_cidr("192.168.1.100", "192.168.1.0", 24));
        assert!(ip_in_cidr("192.168.1.1", "192.168.1.0", 24));
        assert!(!ip_in_cidr("192.168.2.1", "192.168.1.0", 24));
        assert!(ip_in_cidr("10.0.0.1", "10.0.0.0", 8));
    }

    #[test]
    fn test_matches_port_pattern() {
        assert!(matches_port_pattern("80", 80));
        assert!(matches_port_pattern("80,443", 443));
        assert!(matches_port_pattern("1-1024", 80));
        assert!(matches_port_pattern("*", 12345));
        assert!(!matches_port_pattern("80", 443));
    }
}
