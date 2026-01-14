use std::thread;
use std::time::Duration;
use crate::sysmodules::{logging, post, fetch, config, paths};
use crate::network::{dhcp, dns, cportal};
use tauri::AppHandle;

#[cfg(target_os = "windows")]
use std::process::Command;

#[cfg(not(target_os = "windows"))]
use std::process::Command;

fn mask_to_prefix(mask: &str) -> Option<u8> {
    let parts: Vec<&str> = mask.split('.').collect();
    if parts.len() != 4 { return None; }
    let mut bits = 0u8;
    for p in parts {
        if let Ok(v) = p.parse::<u8>() {
            bits += v.count_ones() as u8;
        } else { return None; }
    }
    Some(bits)
}

fn assign_ip_to_interface(interface: &str, ip: &str, mask: &str, gateway: Option<&str>) {
    logging::log_info(&format!("Assigning IP {} mask {} to interface {}", ip, mask, interface));

    #[cfg(target_os = "windows")]
    {
        // netsh expects interface name and ip mask gateway
        let gw = gateway.unwrap_or(ip);
        let _ = Command::new("netsh")
            .args(["interface", "ip", "set", "address", &format!("name=\"{}\"", interface), "static", ip, mask, gw])
            .output()
            .map_err(|e| logging::log_error(&format!("Failed to assign IP (windows): {}", e))).ok();
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(prefix) = mask_to_prefix(mask) {
            let cidr = format!("{}/{}", ip, prefix);
            let _ = Command::new("ip")
                .args(["addr", "add", &cidr, "dev", interface])
                .output()
                .map_err(|e| logging::log_error(&format!("Failed to assign IP (linux): {}", e))).ok();
            let _ = Command::new("ip").args(["link","set","dev",interface,"up"]).output().ok();
            if let Some(gw) = gateway {
                let _ = Command::new("ip").args(["route","add","default","via",gw,"dev",interface]).output().ok();
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // on macOS, use ifconfig
        let _ = Command::new("ifconfig")
            .args([interface, ip, "netmask", mask])
            .output()
            .map_err(|e| logging::log_error(&format!("Failed to assign IP (macos): {}", e))).ok();
        if let Some(gw) = gateway {
            let _ = Command::new("route").args(["add","default",gw]).output().ok();
        }
    }
}

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
            let upstream = cfg.dhcp.upstream_interface.clone();
            logging::log_info(&format!("Setting DNS upstream interface to: {}", upstream));
            dns::set_upstream_interface(upstream);

            // Apply Captive Portal rules if enabled
            if cfg.dhcp.captive_portal {
                logging::log_info("Applying Captive Portal rules on startup...");
                cportal::apply_portal_rules(true, &cfg);
            }

            // If hotspot enabled, try to auto-assign gateway IP to hotspot interface
            if cfg.hotspot.enabled {
                logging::log_info(&format!("Hotspot enabled; attempting to assign gateway {} to interface {}", cfg.dhcp.gateway, cfg.hotspot.interface));
                // Best-effort: assign gateway IP to the configured hotspot interface
                assign_ip_to_interface(&cfg.hotspot.interface, &cfg.dhcp.gateway, &cfg.dhcp.subnet_mask, Some(&cfg.dhcp.gateway));
            }
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

// Tauri command to reload networking after config changes
#[tauri::command]
pub fn reload_networking() {
    logging::log_info("Reloading networking components via reload_networking command...");
    shutdown_networking();
    // Initialize with no AppHandle (callers can restart services with appropriate handle if needed)
    initialize_networking(None);
}
