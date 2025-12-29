use dotenv::dotenv;
use std::{env, fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupConfig {
    pub hostname: String,
    pub admin_email: String,
    pub admin_user: String,
    pub admin_pass: String,
    pub telemetry: bool,
    pub first_run: bool,
}

fn resolve_path(env_key: &str, default: &str) -> PathBuf {
    dotenv().ok();
    let path = env::var(env_key).unwrap_or_else(|_| default.to_string());
    PathBuf::from(path)
}

#[tauri::command]
pub fn load_logging_config() -> Result<LoggingConfig, String> {
    let path = resolve_path("CRABFLOW_LOG_CONFIG", "config/logging.conf.json");
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: LoggingConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn load_setup_config() -> Result<SetupConfig, String> {
    let path = resolve_path("CRABFLOW_SETUP_CONFIG", "config/setup.conf");
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn save_setup_config(config: SetupConfig) -> Result<(), String> {
    let path = resolve_path("CRABFLOW_SETUP_CONFIG", "config/setup.conf");
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_setup_config() -> Result<(), String> {
    let path = resolve_path("CRABFLOW_SETUP_CONFIG", "config/setup.conf");
    let default = SetupConfig {
        hostname: "".into(),
        admin_email: "".into(),
        admin_user: "".into(),
        admin_pass: "".into(),
        telemetry: false,
        first_run: true,
    };
    let json = serde_json::to_string_pretty(&default).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
