use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::sysmodules::paths;

pub fn get_project_root() -> PathBuf {
    paths::get_install_dir()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DhcpConfig {
    pub enabled: bool,
    pub captive_portal: bool,
    #[serde(default)]
    pub custom_captive_portal: bool, // New flag for custom portal editor
    pub bind_address: String, // Interface IP to bind to (e.g. 0.0.0.0 or 192.168.137.1)
    pub upstream_interface: String, // New: Interface IP to use for outbound forwarding
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
            enabled: true,
            captive_portal: false,
            custom_captive_portal: false,
            bind_address: "0.0.0.0".into(),
            upstream_interface: "0.0.0.0".into(),
            range_start: "192.168.1.100".into(),
            range_end: "192.168.1.200".into(),
            subnet_mask: "255.255.255.0".into(),
            gateway: "192.168.1.1".into(),
            dns_servers: vec!["8.8.8.8".into(), "8.8.4.4".into()],
            lease_time: 86400,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HotspotConfig {
    pub enabled: bool,
    pub ssid: String,
    pub password: String,
    pub interface: String, // e.g. "Wi-Fi" or "Ethernet 2"
}

impl Default for HotspotConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            ssid: "CrabFlow-Hotspot".into(),
            password: "password123".into(),
            interface: "Wi-Fi".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsConfig {
    pub allow_non_dhcp_clients: bool,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            allow_non_dhcp_clients: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdvancedConfig {
    pub dns_read_timeout_ms: u64,
    pub dhcp_lease_duration_sec: u64,
    pub captive_portal_domain: String,
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            dns_read_timeout_ms: 2000,
            dhcp_lease_duration_sec: 7200,
            captive_portal_domain: "portal.crabflow.local".into(),
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
    #[serde(default)]
    pub dns: DnsConfig,
    #[serde(default)]
    pub hotspot: HotspotConfig,
    #[serde(default)]
    pub advanced: AdvancedConfig,
}

fn default_monitor_interval() -> u64 {
    5000
}

#[tauri::command]
pub fn load_logging_config() -> Result<LoggingConfig, String> {
    let path = paths::get_config_path("logging.conf.json");
    
    if !path.exists() {
        return Ok(LoggingConfig {
            level: "INFO".to_string(),
            file: "crabflow.log".to_string(),
        });
    }

    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: LoggingConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn save_logging_config(config: LoggingConfig) -> Result<(), String> {
    let path = paths::get_config_path("logging.conf.json");
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_setup_config() -> Result<SetupConfig, String> {
    let path = paths::get_config_path("crabflow_config.json");
    
    if !path.exists() {
        return Ok(SetupConfig {
            hostname: "".into(),
            admin_email: "".into(),
            admin_user: "".into(),
            admin_pass: "".into(),
            telemetry: false,
            first_run: true,
            monitor_interval: 5000,
            dhcp: DhcpConfig {
                enabled: true,
                custom_captive_portal: false,
                ..DhcpConfig::default()
            },
            dns: DnsConfig::default(),
            hotspot: HotspotConfig::default(),
            advanced: AdvancedConfig::default(),
        });
    }

    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn save_setup_config(config: SetupConfig) -> Result<(), String> {
    let path = paths::get_config_path("crabflow_config.json");
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_setup_config() -> Result<(), String> {
    let path = paths::get_config_path("crabflow_config.json");
    let default = SetupConfig {
        hostname: "".into(),
        admin_email: "".into(),
        admin_user: "".into(),
        admin_pass: "".into(),
        telemetry: false,
        first_run: true,
        monitor_interval: 5000,
        dhcp: DhcpConfig {
            enabled: true,
            custom_captive_portal: false,
            ..DhcpConfig::default()
        },
        dns: DnsConfig::default(),
        hotspot: HotspotConfig::default(),
        advanced: AdvancedConfig::default(),
    };
    let json = serde_json::to_string_pretty(&default).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())

}
