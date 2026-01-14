// src-tauri/src/network/cportal.rs

use crate::user_management::user::{UserStore, LoginRecord};
use crate::sysmodules::{fetch, post, notify, paths};
use crate::sysmodules::config::SetupConfig;
use crate::network::dns;
use tauri::{State, AppHandle};
use chrono::Utc;
use std::fs;
// use std::process::Command;

fn get_portal_path() -> std::path::PathBuf {
    paths::get_config_path("portal.html")
}

pub fn apply_portal_rules(_enabled: bool, _config: &SetupConfig) {
    #[cfg(target_os = "linux")]
    {
        // Assume eth0 is LAN for now based on user context
        let lan_int = "eth0"; 
        let gateway = &config.dhcp.gateway;
        let portal_port = "3030";

        let op = if enabled { "-I" } else { "-D" }; // Use Insert to ensure it's at the top

        // 1. Redirect HTTP traffic to Portal
        // iptables -t nat -I PREROUTING -i eth0 -p tcp --dport 80 -j DNAT --to-destination 10.0.0.1:3030
        let _ = Command::new("iptables")
            .args(["-t", "nat", op, "PREROUTING", "-i", lan_int, "-p", "tcp", "--dport", "80", "-j", "DNAT", "--to-destination", &format!("{}:{}", gateway, portal_port)])
            .output();
            
        // 2. Allow access to Portal Port in INPUT chain
        let _ = Command::new("iptables")
            .args([op, "INPUT", "-i", lan_int, "-p", "tcp", "--dport", portal_port, "-j", "ACCEPT"])
            .output();
            
        // 3. For HTTPS, we can't easily decrypt, but we can block it to force fallback to HTTP or let it timeout?
        // Or we can DNAT it to 3030 too, but the browser will show certificate error.
        // Most modern OSes check a specific HTTP URL for captive portal detection.
        // So redirecting port 80 is usually sufficient.
        
        crate::sysmodules::logging::log_info(&format!("Captive Portal rules applied: {} (Interface: {})", enabled, lan_int));
    }
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
    
    // Apply (or remove) rules based on the new state
    apply_portal_rules(enabled, &config);
    
    config.dhcp.captive_portal = enabled;
    post::post_setup(config).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn set_custom_portal(enabled: bool) -> Result<(), String> {
    let mut config: SetupConfig = fetch::fetch_setup().map_err(|e| e.to_string())?;
    config.dhcp.custom_captive_portal = enabled;
    post::post_setup(config).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn tag_user(
    app: AppHandle,
    user_store: State<'_, UserStore>,
    username: String,
    ip: String,
    device_name: Option<String>
) -> Result<(), String> {
    {
        let mut db = user_store.db.lock().map_err(|e| e.to_string())?;
        
        // ... (MAC lookup logic stays same)
        let mac = if ip == "127.0.0.1" || ip == "localhost" {
            "00:00:00:00:00:00".to_string()
        } else {
            match crate::network::dhcp::get_mac_from_ip(&ip) {
                Some(m) => m,
                None => {
                    crate::sysmodules::logging::log_warn(&format!("Tagging user {} with unknown MAC (IP: {})", username, ip));
                    "UNKNOWN_MAC".to_string()
                }
            }
        };

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
            let err = format!("User not found: {}", username);
            notify::send_notification(&app, "Authentication Failed", &err, "error");
            return Err(err);
        }
    }
    
    // Whitelist the user so DNS stops hijacking them
    dns::authorize_ip(ip.clone());

    notify::send_notification(&app, "User Authenticated", &format!("User: {}\nIP: {}", username, ip), "success");
    user_store.persist().await.map_err(|e| e.to_string())
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
