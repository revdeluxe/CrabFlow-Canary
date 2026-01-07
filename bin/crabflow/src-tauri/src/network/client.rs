// src-tauri/src/client.rs

// use tauri::Manager;

// Import your feature modules
use crate::network::dhcp;
use crate::network::dns;
use crate::sysmodules::{logging, fetch, post};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub mac: String,
    pub ip: String,
    pub hostname: String,
    pub status: String,
}

// Re-export structs for frontend use
pub use dhcp::{Lease, LeaseInput};
pub use dns::{DnsRecord, DnsRecordInput};
// pub use logging::log_event;

/// DHCP commands
#[tauri::command]
pub fn list_leases() -> Vec<Lease> {
    dhcp::list_leases()
}

#[tauri::command]
pub fn add_static_lease(input: LeaseInput) -> Result<(), String> {
    dhcp::add_static_lease(input)
}

#[tauri::command]
pub fn remove_lease(ip: String) -> Result<(), String> {
    dhcp::remove_lease(ip)
}

/// DNS commands
#[tauri::command]
pub fn list_records() -> Vec<DnsRecord> {
    dns::list_records()
}

#[tauri::command]
pub fn add_record(input: DnsRecordInput) -> Result<(), String> {
    dns::add_record(input)
}

#[tauri::command]
pub fn update_record(old_name: String, old_rtype: String, input: DnsRecordInput) -> Result<(), String> {
    dns::update_record(old_name, old_rtype, input)
}

#[tauri::command]
pub fn remove_record(name: String, rtype: String) -> Result<(), String> {
    dns::remove_record(name, rtype)
}

#[tauri::command]
pub fn update_upstream_interface(ip: String) {
    dns::set_upstream_interface(ip);
}

/// Logging commands
#[tauri::command]
pub fn log_action(actor: String, action: String, target: String) {
    logging::log_event(actor, action, target);
}

/// Example FS fetch/post wrappers
#[tauri::command]
pub fn fetch_config(path: String) -> Result<String, String> {
    fetch::read_file(&path)
}

#[tauri::command]
pub fn save_config(path: String, data: String) -> Result<(), String> {
    post::write_file(&path, &data)
}

#[tauri::command]
pub fn list_devices() -> Vec<Device> {
    // Mock implementation for now
    vec![
        Device {
            mac: "00:11:22:33:44:55".to_string(),
            ip: "192.168.1.10".to_string(),
            hostname: "Device-1".to_string(),
            status: "Online".to_string(),
        }
    ]
}
