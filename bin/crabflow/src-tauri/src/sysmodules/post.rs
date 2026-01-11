// src-tauri/src/sysmodules/post.rs

use std::path::PathBuf;
use std::fs;
use crate::sysmodules::config::SetupConfig;
use crate::sysmodules::paths;

fn config_path() -> PathBuf {
    paths::get_config_path("crabflow_config.json")
}

/// Write a file safely to the appropriate directory
pub fn write_file(filename: &str, data: &str) -> Result<(), String> {
    // Decide where to write based on filename
    // leases.json, system_stats.json -> db
    // others -> config
    
    let path = if filename == "leases.json" || filename == "system_stats.json" {
        paths::get_db_path(filename)
    } else {
        paths::get_config_path(filename)
    };

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(&path, data).map_err(|e| format!("Failed to write file: {}", e))
}

/// Append data to a file safely
#[allow(dead_code)]
pub fn append_file(filename: &str, data: &str) -> Result<(), String> {
    let path = if filename == "leases.json" || filename == "system_stats.json" {
        paths::get_db_path(filename)
    } else {
        paths::get_config_path(filename)
    };

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let mut content = fs::read_to_string(&path).unwrap_or_default();
    content.push_str(data);

    fs::write(&path, content).map_err(|e| format!("Failed to append file: {}", e))
}

#[tauri::command]
pub fn post_setup(config: SetupConfig) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
