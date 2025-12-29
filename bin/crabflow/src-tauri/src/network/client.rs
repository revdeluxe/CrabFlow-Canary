// src-tauri/src/client.rs

use tauri::Manager;

// Import your feature modules
mod dhcp;
mod dns;
mod logging;
mod fetch;
mod post;

// Re-export structs for frontend use
pub use dhcp::{Lease, LeaseInput};
pub use dns::{DnsRecord, DnsRecordInput};
pub use logging::log_event;

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
pub fn remove_record(name: String, rtype: String) -> Result<(), String> {
    dns::remove_record(name, rtype)
}

/// Logging commands
#[tauri::command]
pub fn log_action(actor: String, action: String, target: String) {
    logging::log_event(actor, action, target);
}

/// Example FS fetch/post wrappers
#[tauri::command]
pub fn fetch_config(path: String) -> Result<String, String> {
    fetch::read_file(path)
}

#[tauri::command]
pub fn save_config(path: String, data: String) -> Result<(), String> {
    post::write_file(path, data)
}
