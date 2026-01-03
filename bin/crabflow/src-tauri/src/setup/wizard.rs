// src-tauri/src/setup/wizard.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dotenv::var;
use crate::sysmodules::config::{get_project_root, DhcpConfig, SetupConfig};

fn default_monitor_interval() -> u64 {
    5000
}

/// Strict config content checker
#[tauri::command]
pub fn validate_config(config: SetupConfig) -> bool {
    !config.hostname.is_empty()
        && !config.admin_email.is_empty()
        && !config.admin_user.is_empty()
        && !config.admin_pass.is_empty()
}


/// Resolve config path under OS-specific config dir
fn config_path() -> PathBuf {
    let root = get_project_root();
    let config_dir = var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let config_file = var("CRABFLOW_CONFIG").unwrap_or_else(|_| "crabflow_config.json".to_string());
    root.join(config_dir).join(config_file)
}

/// Save setup config to disk
#[tauri::command]
pub fn save_setup(config: SetupConfig) -> Result<(), String> {
    let path = config_path();
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Load setup config from disk
#[tauri::command]
pub fn load_setup() -> Result<SetupConfig, String> {
    let path = config_path();
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

/// Check if setup is required (first run or missing config)
#[tauri::command]
pub fn check_setup() -> bool {
    match load_setup() {
        Ok(cfg) => cfg.first_run,
        Err(_) => true, // no config → setup required
    }
}
/// Reset setup config (for testing)
#[tauri::command]
pub fn reset_setup() -> Result<(), String> {
    let path = config_path();
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// First run check
#[tauri::command]
pub fn check_first_run() -> bool {
    match load_setup() {
        Ok(cfg) => cfg.first_run,
        Err(_) => true, // no config → first run
    }
}