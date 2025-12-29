use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;

#[derive(Serialize, Deserialize)]
pub struct SetupConfig {
    pub hostname: String,
    pub admin_email: String,
    pub admin_user: String,
    pub admin_pass: String, // ⚠️ later replace with hashed password
    pub telemetry: bool,
    pub first_run: bool,
}

fn config_path() -> PathBuf {
    // Save under CrabFlow config directory
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("crabflow_config.json");
    path
}

#[tauri::command]
pub fn save_setup(config: SetupConfig) -> Result<(), String> {
    let path = config_path();
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

use crate::sysmodules::config::load_setup_config;

#[tauri::command]
pub fn check_first_run() -> bool {
    match load_setup_config() {
        Ok(cfg) => cfg.first_run,
        Err(_) => true, // fallback: assume first run
    }
}

#[tauri::command]
pub fn load_setup() -> Result<SetupConfig, String> {
    let path = config_path();
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}
