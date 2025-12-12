// src/client_list/mod.rs
// Manages the collection of all monitored clients.

use std::collections::HashMap;
use crate::client_list::client::{Client, ClientStatus};

pub mod client;

/// The main structure for managing all network clients.
#[derive(Debug)]
pub struct ClientList {
    clients: HashMap<u32, Client>, // Keyed by Client ID
}

impl ClientList {
    /// Creates a new ClientList and initializes it with dummy data for testing.
    pub fn new() -> Self {
        let mut clients = HashMap::new();

        let c1 = Client {
            id: 101,
            hostname: "web-server-01".to_string(),
            ip_address: "192.168.1.50".to_string(),
            status: ClientStatus::Online,
            last_seen: "Just now".to_string(),
        };
        clients.insert(c1.id, c1);

        let c2 = Client {
            id: 102,
            hostname: "client-laptop-A".to_string(),
            ip_address: "192.168.1.102".to_string(),
            status: ClientStatus::Offline,
            last_seen: "5 minutes ago".to_string(),
        };
        clients.insert(c2.id, c2);

        let c3 = Client {
            id: 103,
            hostname: "router-gw".to_string(),
            ip_address: "192.168.1.1".to_string(),
            status: ClientStatus::Monitoring,
            last_seen: "Just now".to_string(),
        };
        clients.insert(c3.id, c3);

        ClientList { clients }
    }

    /// Retrieves a list of all clients.
    pub fn get_all_clients(&self) -> Vec<Client> {
        self.clients.values().cloned().collect()
    }

    /// Finds a client by its ID.
    pub fn get_client_by_id(&self, id: u32) -> Option<&Client> {
        self.clients.get(&id)
    }
}