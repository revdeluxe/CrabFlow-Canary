// src-tauri/src/sysmodules/fetch.rs

use std::path::PathBuf;
use std::fs;
// use serde::Deserialize;
use dotenv::var;
use crate::sysmodules::config::{get_project_root, SetupConfig};

/// Read a file safely from the config directory
pub fn read_file(filename: &str) -> Result<String, String> {
    // Determine if it's a config file or db file based on extension or name?
    // Or just check both? Or rely on absolute path if provided?
    // The user wants specific directories.
    // Let's try to find the file in config dir first, then db dir.
    
    let root = get_project_root();
    let config_dir_name = var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let db_dir_name = var("CRABFLOW_DB_DIR").unwrap_or_else(|_| "db".to_string());

    let config_path = root.join(&config_dir_name).join(filename);
    if config_path.exists() {
        return fs::read_to_string(config_path).map_err(|e| format!("Failed to read file: {}", e));
    }

    let db_path = root.join(&db_dir_name).join(filename);
    if db_path.exists() {
        return fs::read_to_string(db_path).map_err(|e| format!("Failed to read file: {}", e));
    }

    // If not found, try to read as is (maybe absolute path or just filename to be created later)
    // But for reading, it must exist.
    // If it doesn't exist, we return error.
    // But wait, init.rs checks if it exists by reading it.
    
    // Let's default to config dir if not found, so the error message reflects that.
    fs::read_to_string(config_path).map_err(|e| format!("Failed to read file: {}", e))
}

fn config_path() -> PathBuf {
    let root = get_project_root();
    let config_dir_name = var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let config_file = var("CRABFLOW_CONFIG").unwrap_or_else(|_| "crabflow_config.json".to_string());
    root.join(config_dir_name).join(config_file)
}

#[tauri::command]
pub fn fetch_setup() -> Result<SetupConfig, String> {
    let path = config_path();
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}
