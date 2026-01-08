// src-tauri/src/setup/wizard.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dotenv::var;
use crate::sysmodules::config::{get_project_root, DhcpConfig, SetupConfig};
use crate::sysmodules::paths;

#[derive(Serialize)]
pub struct WizardStatus {
    pub config: Option<SetupConfig>,
    pub data_location: String,
    pub is_configured: bool,
}

#[tauri::command]
pub fn get_wizard_status() -> Result<WizardStatus, String> {
    let path = paths::get_config_path("crabflow_config.json");
    let data_location = paths::get_data_dir().to_string_lossy().to_string();
    
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(data) => {
                match serde_json::from_str::<SetupConfig>(&data) {
                    Ok(config) => Ok(WizardStatus {
                        is_configured: !config.first_run,
                        config: Some(config),
                        data_location,
                    }),
                    Err(_) => Ok(WizardStatus {
                        is_configured: false,
                        config: None,
                        data_location,
                    })
                }
            },
            Err(_) => Ok(WizardStatus {
                is_configured: false,
                config: None,
                data_location,
            })
        }
    } else {
         Ok(WizardStatus {
            is_configured: false,
            config: None,
            data_location,
        })
    }
}

/// Strict config content checker
#[tauri::command]
pub fn validate_config(config: SetupConfig) -> bool {
    !config.hostname.is_empty()
        && !config.admin_email.is_empty()
        && !config.admin_user.is_empty()
        && !config.admin_pass.is_empty()
}


/// Resolve config path under OS-specific config dir
fn config_path() -> PathBuf {
    paths::get_config_path("crabflow_config.json")
}

/// Save setup config to disk
#[tauri::command]
pub fn save_setup(config: SetupConfig) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Load setup config from disk
#[tauri::command]
pub fn load_setup() -> Result<SetupConfig, String> {
    let path = config_path();
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: SetupConfig = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(config)
}

/// Check if setup is required (first run or missing config)
#[tauri::command]
pub fn check_setup() -> bool {
    match load_setup() {
        Ok(cfg) => cfg.first_run,
        Err(_) => true, // no config → setup required
    }
}
/// Reset setup config (for testing)
#[tauri::command]
pub fn reset_setup() -> Result<(), String> {
    let path = config_path();
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// First run check
#[tauri::command]
pub fn check_first_run() -> bool {
    match load_setup() {
        Ok(cfg) => cfg.first_run,
        Err(_) => true, // no config → first run
    }
}