use crate::sysmodules::{logging, post, fetch, config, paths};
use crate::network::{dhcp, dns};
use tauri::AppHandle;
use dotenv::var;
use std::thread;
use std::time::Duration;
use std::process::Command;

#[cfg(target_os = "windows")]
fn prepare_windows_environment() {
    logging::log_info("Preparing Windows environment...");

    // Step A: Admin Check
    let output = Command::new("net")
        .arg("session")
        .output();
    
    match output {
        Ok(o) if o.status.success() => {
             logging::log_info("Admin privileges confirmed.");
        },
        _ => {
            logging::log_error("CRITICAL: Application is NOT running as Administrator. Network services may fail.");
        }
    }

    // Step B: Kill ICS (SharedAccess) to free port 53/67
    logging::log_info("Stopping SharedAccess service...");
    let _ = Command::new("net")
        .args(["stop", "SharedAccess"])
        .output();

    // Step C: Enable IP Routing
    logging::log_info("Enabling IP EnableRouter...");
    let _ = Command::new("reg")
        .args(["add", "HKLM\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters", "/v", "IPEnableRouter", "/t", "REG_DWORD", "/d", "1", "/f"])
        .output();
        
    logging::log_info("Windows environment preparation complete.");
}

pub fn initialize_networking(app_handle: Option<AppHandle>) {
    logging::log_info("Initializing networking components...");
    
    // Ensure data directory exists
    if let Err(e) = paths::init_data_dir() {
        logging::log_error(&format!("Failed to initialize data directory: {}", e));
    }

    #[cfg(target_os = "windows")]
    {
        prepare_windows_environment();
        // Wait a bit for services to stop/registry to apply
        thread::sleep(Duration::from_secs(2));
    }

    let leases_file = paths::get_config_path("leases.json").to_string_lossy().to_string();
    let dns_file = paths::get_config_path("dns.json").to_string_lossy().to_string();

    // Load main configuration to set upstream interface
    match config::load_setup_config() {
        Ok(cfg) => {
            let upstream = cfg.dhcp.upstream_interface;
            logging::log_info(&format!("Setting DNS upstream interface to: {}", upstream));
            dns::set_upstream_interface(upstream);
        },
        Err(e) => {
            logging::log_error(&format!("Failed to load setup config: {}", e));
        }
    }

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
    dhcp::start_dhcp_server(app_handle.clone());
    
    // Start DNS Server in a background thread
    dns::start_dns_server(app_handle);

    logging::log_info("Networking initialization complete.");
}

pub fn shutdown_networking() {
    logging::log_info("Shutting down networking components...");
    dhcp::stop_dhcp_server();
    dns::stop_dns_server();
    logging::log_info("Networking shutdown complete.");
}
