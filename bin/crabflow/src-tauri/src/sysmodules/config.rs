use dotenv::dotenv;
use std::{env, fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;

pub fn get_project_root() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.ends_with("src-tauri") {
        return cwd.parent().unwrap().to_path_buf();
    }
    cwd
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DhcpConfig {
    pub enabled: bool,
    pub captive_portal: bool, // New field
    pub range_start: String,
    pub range_end: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns_servers: Vec<String>,
    pub lease_time: u64,
}

impl Default for DhcpConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            captive_portal: false,
            range_start: "192.168.1.100".into(),
            range_end: "192.168.1.200".into(),
            subnet_mask: "255.255.255.0".into(),
            gateway: "192.168.1.1".into(),
            dns_servers: vec!["8.8.8.8".into(), "8.8.4.4".into()],
            lease_time: 86400,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupConfig {
    pub hostname: String,
    pub admin_email: String,
    pub admin_user: String,
    pub admin_pass: String,
    pub telemetry: bool,
    pub first_run: bool,
    #[serde(default = "default_monitor_interval")]
    pub monitor_interval: u64,
    #[serde(default)]
    pub dhcp: DhcpConfig,
}

fn default_monitor_interval() -> u64 {
    5000
}

fn resolve_path(env_key: &str, default: &str) -> PathBuf {
    dotenv().ok();
    let path = env::var(env_key).unwrap_or_else(|_| default.to_string());
    PathBuf::from(path)
}

#[tauri::command]
pub fn load_logging_config() -> Result<LoggingConfig, String> {
    let config_dir = env::var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let default_path = format!("{}/logging.conf.json", config_dir);
    let path = resolve_path("CRABFLOW_LOG_CONFIG", &default_path);
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: LoggingConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn load_setup_config() -> Result<SetupConfig, String> {
    let config_dir = env::var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let default_path = format!("{}/setup.conf", config_dir);
    let path = resolve_path("CRABFLOW_SETUP_CONFIG", &default_path);
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn save_setup_config(config: SetupConfig) -> Result<(), String> {
    let config_dir = env::var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let default_path = format!("{}/setup.conf", config_dir);
    let path = resolve_path("CRABFLOW_SETUP_CONFIG", &default_path);
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
        monitor_interval: 5000,
        dhcp: DhcpConfig::default(),
    };
    let json = serde_json::to_string_pretty(&default).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
