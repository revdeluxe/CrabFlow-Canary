use crate::sysmodules::{logging, post, fetch, config, paths};
use crate::network::{dhcp, dns};
use tauri::AppHandle;

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "linux")]
fn prepare_linux_environment() {
    use std::process::Command;
    
    logging::log_info("Preparing Linux environment...");

    // Check for root privileges
    let uid = unsafe { libc::getuid() };
    if uid != 0 {
        logging::log_error("CRITICAL: Application is NOT running as root. Network services may fail.");
    } else {
        logging::log_info("Root privileges confirmed.");
    }

    // Enable IP forwarding
    logging::log_info("Enabling IP forwarding...");
    let _ = Command::new("sysctl")
        .args(["-w", "net.ipv4.ip_forward=1"])
        .output();

    logging::log_info("Linux environment preparation complete.");
}

#[cfg(target_os = "macos")]
fn prepare_macos_environment() {
    use std::process::Command;
    
    logging::log_info("Preparing macOS environment...");

    // Check for root privileges
    let uid = unsafe { libc::getuid() };
    if uid != 0 {
        logging::log_error("CRITICAL: Application is NOT running as root. Network services may fail.");
    } else {
        logging::log_info("Root privileges confirmed.");
    }

    // Enable IP forwarding
    logging::log_info("Enabling IP forwarding...");
    let _ = Command::new("sysctl")
        .args(["-w", "net.inet.ip.forwarding=1"])
        .output();

    logging::log_info("macOS environment preparation complete.");
}

pub fn initialize_networking(app_handle: Option<AppHandle>) {
    logging::log_info("Initializing networking components...");
    
    // Ensure data directories exist
    if let Err(e) = paths::init_data_dir() {
        logging::log_error(&format!("Failed to initialize data directory: {}", e));
    }

    // Platform-specific environment preparation
    #[cfg(target_os = "windows")]
    {
        prepare_windows_environment();
        // Wait a bit for services to stop/registry to apply
        thread::sleep(Duration::from_secs(2));
    }

    #[cfg(target_os = "linux")]
    {
        prepare_linux_environment();
    }

    #[cfg(target_os = "macos")]
    {
        prepare_macos_environment();
    }

    // Use new paths module for file locations
    let leases_file = paths::get_db_path("leases.json");
    let dns_file = paths::get_config_path("dns.json");

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
    let leases_str = leases_file.to_string_lossy().to_string();
    match fetch::read_file(&leases_str) {
        Ok(_) => logging::log_info("DHCP configuration found."),
        Err(_) => {
            logging::log_info("DHCP configuration not found. Creating default.");
            if let Err(e) = post::write_file(&leases_str, "[]") {
                logging::log_error(&format!("Failed to create {}: {}", leases_str, e));
            }
        }
    }

    // Initialize DNS
    let dns_str = dns_file.to_string_lossy().to_string();
    match fetch::read_file(&dns_str) {
        Ok(_) => logging::log_info("DNS configuration found."),
        Err(_) => {
            logging::log_info("DNS configuration not found. Creating default.");
            if let Err(e) = post::write_file(&dns_str, "[]") {
                logging::log_error(&format!("Failed to create {}: {}", dns_str, e));
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
