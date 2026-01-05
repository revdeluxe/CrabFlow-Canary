// src-tauri/src/sysmodules/post.rs

use std::path::PathBuf;
use std::fs;
// use serde::{Serialize, Deserialize};
use dotenv::var;
use crate::sysmodules::config::{get_project_root, SetupConfig};

fn config_path() -> PathBuf {
    let root = get_project_root();
    let config_dir_name = var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let config_file = var("CRABFLOW_CONFIG").unwrap_or_else(|_| "crabflow_config.json".to_string());
    root.join(config_dir_name).join(config_file)
}

/// Write a file safely to the config directory
pub fn write_file(filename: &str, data: &str) -> Result<(), String> {
    // Decide where to write based on filename or extension?
    // leases.json -> db
    // dns.json -> config
    // others -> config
    
    let root = get_project_root();
    let config_dir_name = var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let db_dir_name = var("CRABFLOW_DB_DIR").unwrap_or_else(|_| "db".to_string());

    let path = if filename == "leases.json" || filename == "system_stats.json" {
        root.join(db_dir_name).join(filename)
    } else {
        root.join(config_dir_name).join(filename)
    };

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(&path, data).map_err(|e| format!("Failed to write file: {}", e))
}

/// Append data to a file safely
pub fn append_file(filename: &str, data: &str) -> Result<(), String> {
    let root = get_project_root();
    let config_dir_name = var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let db_dir_name = var("CRABFLOW_DB_DIR").unwrap_or_else(|_| "db".to_string());

    let path = if filename == "leases.json" || filename == "system_stats.json" {
        root.join(db_dir_name).join(filename)
    } else {
        root.join(config_dir_name).join(filename)
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
