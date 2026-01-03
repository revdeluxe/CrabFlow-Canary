// src-tauri/src/init.rs
use std::path::Path;
use dotenv::var; // or `use dotenv::var;` if you already use dotenv

/// Verify that all required system files exist
pub fn does_system_files() -> bool {
    dotenv::dotenv().ok(); // load .env

    // Directories from .env
    let network = var("CRABFLOW_NETWORK_DIR").unwrap_or_else(|_| "src-tauri/src/network".into());
    let setup = var("CRABFLOW_SETUP_DIR").unwrap_or_else(|_| "src-tauri/src/setup".into());
    let sysmodules = var("CRABFLOW_SYSMODULES_DIR").unwrap_or_else(|_| "src-tauri/src/sysmodules".into());
    let umanager = var("CRABFLOW_USER_DIR").unwrap_or_else(|_| "src-tauri/src/user_management".into());

    // Expected files in each directory
    let network_attributes = [
        "cportal.rs", "mod.rs", "dns.rs", "dhcp.rs", "firewall.rs", "monitor.rs", "auth.rs", "client.rs", "init.rs",
    ];
    let setup_attributes = ["wizard.rs", "mod.rs"];
    let sysmodules_attributes = ["config.rs", "fetch.rs", "logging.rs", "post.rs", "mod.rs"];
    let umanager_attributes = ["user.rs", "auth.rs", "mod.rs", "permission.rs"];

    // Check each directory and its files
    let dirs_to_check: Vec<(String, &[&str])> = vec![
        (network, &network_attributes),
        (setup, &setup_attributes),
        (sysmodules, &sysmodules_attributes),
        (umanager, &umanager_attributes),
    ];

    for (dir, attrs) in dirs_to_check {
        if !Path::new(&dir).exists() {
            return false;
        }
        for attr in attrs {
            let file_path = format!("{}/{}", dir, attr);
            if !Path::new(&file_path).exists() {
                return false;
            }
        }
    }
    true
}
