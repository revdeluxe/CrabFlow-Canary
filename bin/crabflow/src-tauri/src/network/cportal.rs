// src-tauri/src/network/cportal.rs

use crate::user_management::user::{UserStore, LoginRecord};
use crate::sysmodules::fetch;
use crate::sysmodules::post;
use crate::sysmodules::config::SetupConfig;
use tauri::State;
use chrono::Utc;
use std::fs;
use dotenv::var;

fn get_portal_path() -> std::path::PathBuf {
    let config_dir = var("CRABFLOW_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let root = crate::sysmodules::config::get_project_root();
    root.join(config_dir).join("portal.html")
}

#[tauri::command]
pub fn get_portal_template() -> Result<String, String> {
    let path = get_portal_path();
    if path.exists() {
        fs::read_to_string(path).map_err(|e| e.to_string())
    } else {
        // Return default template if file doesn't exist
        Ok(r#"
<div class="container" style="height: 100vh; display: flex; align-items: center; justify-content: center; flex-direction: column; font-family: sans-serif;">
  <h1>Welcome to CrabFlow Network</h1>
  <p>Please sign in to access the internet.</p>
  
  <div class="card" style="width: 100%; max-width: 400px; margin-top: 2rem; padding: 2rem; border: 1px solid #ccc; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
    <form id="login-form" onsubmit="handleLogin(event)">
      <label style="display: block; margin-bottom: 0.5rem;">Username / Voucher</label>
      <input type="text" id="username" name="username" placeholder="Enter code" style="width: 100%; padding: 0.5rem; margin-bottom: 1rem;" required />
      
      <label style="display: block; margin-bottom: 0.5rem;">Password</label>
      <input type="password" id="password" name="password" placeholder="Enter password" style="width: 100%; padding: 0.5rem; margin-bottom: 1rem;" required />

      <button type="submit" style="width: 100%; padding: 0.75rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">Connect</button>
    </form>
    <div id="error-message" style="color: red; margin-top: 1rem; display: none;"></div>
  </div>
</div>
        "#.to_string())
    }
}

#[tauri::command]
pub fn save_portal_template(content: String) -> Result<(), String> {
    let path = get_portal_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_captive_portal(enabled: bool) -> Result<(), String> {
    let mut config: SetupConfig = fetch::fetch_setup().map_err(|e| e.to_string())?;
    config.dhcp.captive_portal = enabled;
    post::post_setup(config).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn tag_user(
    user_store: State<'_, UserStore>,
    username: String,
    ip: String,
    mac: String,
    device_name: Option<String>
) -> Result<(), String> {
    {
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
        } else {
            return Err("User not found".to_string());
        }
    }
    
    // Save database
    user_store.persist().await?;
    
    Ok(())
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
