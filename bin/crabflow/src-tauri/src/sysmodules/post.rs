// src-tauri/src/sysmodules/post.rs

use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use dirs::config_dir;

#[derive(Serialize, Deserialize)]
pub struct SetupConfig {
    pub hostname: String,
    pub admin_email: String,
    pub admin_user: String,
    pub admin_pass: String, // ⚠️ hash later
    pub telemetry: bool,
    pub first_run: bool,
}

fn config_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("crabflow_config.json");
    path
}

/// Write a file safely to the config directory
pub fn write_file(filename: &str, data: &str) -> Result<(), String> {
    let base: PathBuf = config_dir().ok_or("Could not resolve config directory")?;
    let path = base.join(filename);
    fs::write(&path, data).map_err(|e| format!("Failed to write file: {}", e))
}

/// Append data to a file safely
pub fn append_file(filename: &str, data: &str) -> Result<(), String> {
    let base: PathBuf = config_dir().ok_or("Could not resolve config directory")?;
    let path = base.join(filename);

    let mut content = fs::read_to_string(&path).unwrap_or_default();
    content.push_str(data);

    fs::write(&path, content).map_err(|e| format!("Failed to append file: {}", e))
}

#[tauri::command]
pub fn post_setup(config: SetupConfig) -> Result<(), String> {
    let path = config_path();
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
