// src-tauri/src/sysmodules/fetch.rs

use std::path::PathBuf;
use std::fs;
use crate::sysmodules::config::SetupConfig;
use crate::sysmodules::paths;

/// Read a file from the data directory
pub fn read_file(filename: &str) -> Result<String, String> {
    let path = paths::get_data_dir().join(filename);
    
    if path.exists() {
        return fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e));
    }
    
    // Fallback: Check config directory relative availability for legacy/dev support?
    // For now, adhere to paths module for consistency
    Err(format!("File not found at {}", path.display()))
}

#[tauri::command]
pub fn fetch_setup() -> Result<SetupConfig, String> {
    let path = paths::get_config_path("crabflow_config.json");
    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config from {}: {}", path.display(), e))?;
    let config: SetupConfig = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    Ok(config)
}
