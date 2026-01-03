use crate::sysmodules::{logging, post, fetch};
use crate::network::{dhcp, dns};
use dotenv::var;
use std::thread;

pub fn initialize_networking() {
    logging::log_info("Initializing networking components...");

    let leases_file = var("CRABFLOW_DHCP_CONFIG").unwrap_or_else(|_| "leases.json".to_string());
    let dns_file = var("CRABFLOW_DNS_CONFIG").unwrap_or_else(|_| "dns.json".to_string());

    // Initialize DHCP
    match fetch::read_file(&leases_file) {
        Ok(_) => logging::log_info("DHCP configuration found."),
        Err(_) => {
            logging::log_info("DHCP configuration not found. Creating default.");
            // leases.json should go to db, but write_file handles it based on name now
            if let Err(e) = post::write_file(&leases_file, "[]") {
                logging::log_error(&format!("Failed to create {}: {}", leases_file, e));
            }
        }
    }

    // Initialize DNS
    match fetch::read_file(&dns_file) {
        Ok(_) => logging::log_info("DNS configuration found."),
        Err(_) => {
            logging::log_info("DNS configuration not found. Creating default.");
            if let Err(e) = post::write_file(&dns_file, "[]") {
                logging::log_error(&format!("Failed to create {}: {}", dns_file, e));
            }
        }
    }

    // Start DHCP Server in a background thread
    dhcp::start_dhcp_server();
    
    // Start DNS Server in a background thread
    dns::start_dns_server();

    logging::log_info("Networking initialization complete.");
}

pub fn shutdown_networking() {
    logging::log_info("Shutting down networking components...");
    dhcp::stop_dhcp_server();
    dns::stop_dns_server();
    logging::log_info("Networking shutdown complete.");
}
