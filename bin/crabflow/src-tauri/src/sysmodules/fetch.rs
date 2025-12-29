// src-tauri/src/sysmodules/fetch.rs

use std::path::PathBuf;
use std::fs;
use dirs::config_dir;
use serde::Deserialize;
use super::post::SetupConfig;

/// Read a file safely from the config directory
pub fn read_file(filename: &str) -> Result<String, String> {
    let path = PathBuf::from(filename);
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))
}

fn config_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("crabflow_config.json");
    path
}

#[tauri::command]
pub fn fetch_setup() -> Result<SetupConfig, String> {
    let path = config_path();
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}
