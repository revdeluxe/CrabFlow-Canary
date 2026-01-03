// src-tauri/src/network/cportal.rs

use crate::user_management::user::{UserStore, LoginRecord};
use crate::sysmodules::fetch;
use crate::sysmodules::post;
use crate::sysmodules::config::SetupConfig;
use tauri::State;
use chrono::Utc;

#[tauri::command]
pub fn set_captive_portal(enabled: bool) -> Result<(), String> {
    let mut config: SetupConfig = fetch::fetch_setup().map_err(|e| e.to_string())?;
    config.dhcp.captive_portal = enabled;
    post::post_setup(config).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn tag_user(
    user_store: State<UserStore>,
    username: String,
    ip: String,
    mac: String,
    device_name: Option<String>
) -> Result<(), String> {
    let mut db = user_store.db.lock().map_err(|e| e.to_string())?;
    
    if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
        let now = Utc::now();
        
        let record = LoginRecord {
            ip: ip.clone(),
            mac: mac.clone(),
            timestamp: now.to_rfc3339(),
            device_name,
        };
        
        user.login_history.push(record);
        
        // Save database
        let content = serde_json::to_string_pretty(&*db).map_err(|e| e.to_string())?;
        std::fs::write(&user_store.db_path, content).map_err(|e| e.to_string())?;
        
        Ok(())
    } else {
        Err("User not found".to_string())
    }
}

#[tauri::command]
pub fn get_user_history(
    user_store: State<UserStore>,
    username: String
) -> Result<Vec<LoginRecord>, String> {
    let db = user_store.db.lock().map_err(|e| e.to_string())?;
    
    if let Some(user) = db.users.iter().find(|u| u.username == username) {
        Ok(user.login_history.clone())
    } else {
        Err("User not found".to_string())
    }
}
