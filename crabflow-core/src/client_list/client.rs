// src/client_list/client.rs
// Defines the structure for a monitored network client.

use serde::{Serialize, Deserialize};

/// Represents a single monitored network client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: u32,
    pub hostname: String,
    pub ip_address: String,
    pub status: ClientStatus,
    pub last_seen: String, // Use a proper time library for a real application
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientStatus {
    Online,
    Offline,
    Monitoring,
    Error,
}