// src-tauri/src/dhcp.rs

use serde::{Serialize, Deserialize};
use crate::{fetch, post, logging};

#[derive(Serialize)]
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

/// List all DHCP leases
pub fn list_leases() -> Vec<Lease> {
    // Example: read from JSON file using fetch.rs
    match fetch::read_file("leases.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => vec![],
    }
}

/// Add a static lease
pub fn add_static_lease(input: LeaseInput) -> Result<(), String> {
    let mut leases = list_leases();
    leases.push(Lease {
        ip: input.ip,
        mac: input.mac,
        hostname: input.hostname,
        expires_at: "never".into(),
        static_lease: true,
    });

    let serialized = serde_json::to_string_pretty(&leases).map_err(|e| e.to_string())?;
    post::write_file("leases.json".into(), serialized)?;
    logging::log_event("system".into(), "add_static_lease".into(), input.ip);
    Ok(())
}

/// Remove a lease by IP
pub fn remove_lease(ip: String) -> Result<(), String> {
    let mut leases = list_leases();
    leases.retain(|l| l.ip != ip);

    let serialized = serde_json::to_string_pretty(&leases).map_err(|e| e.to_string())?;
    post::write_file("leases.json".into(), serialized)?;
    logging::log_event("system".into(), "remove_lease".into(), ip);
    Ok(())
}
