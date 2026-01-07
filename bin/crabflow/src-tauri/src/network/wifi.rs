// src-tauri/src/network/wifi.rs
use std::process::Command;
use crate::sysmodules::logging;
use std::io::Write;

/// Create a hotspot (Windows Hosted Network)
/// Note: Hosted Network is deprecated in newer Windows 10/11, but this is the standard command way.
/// Modern way requires WinRT APIs which are harder to call from simple Rust without crates.
#[tauri::command]
pub fn create_hotspot(ssid: String, key: String) -> Result<(), String> {
    logging::log_info(&format!("Attempting to create hotspot: {}", ssid));

    // 1. Set hosted network
    let output_set = Command::new("netsh")
        .args(["wlan", "set", "hostednetwork", "mode=allow", &format!("ssid={}", ssid), &format!("key={}", key)])
        .output()
        .map_err(|e| e.to_string())?;

    if !output_set.status.success() {
        return Err(format!("Failed to set hosted network: {}", String::from_utf8_lossy(&output_set.stderr)));
    }

    // 2. Start hosted network
    let output_start = Command::new("netsh")
        .args(["wlan", "start", "hostednetwork"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output_start.status.success() {
        return Err(format!("Failed to start hosted network: {}", String::from_utf8_lossy(&output_start.stderr)));
    }

    // 3. Start Anti-Orphan Watchdog
    start_watchdog();

    logging::log_info("Hotspot started successfully.");
    Ok(())
}

fn start_watchdog() {
    let pid = std::process::id();
    let script = format!(
        r#"
        param($TargetPid)
        while (Get-Process -Id $TargetPid -ErrorAction SilentlyContinue) {{
            Start-Sleep -Seconds 5
        }}
        # Parent process ($TargetPid) is gone. Stopping hotspot.
        netsh wlan stop hostednetwork
        "#,
    );

    // Create a temporary script file
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("crabflow_wifi_watchdog.ps1");
    
    if let Ok(mut file) = std::fs::File::create(&temp_dir) {
        if file.write_all(script.as_bytes()).is_ok() {
             // Spawn detached powershell
             let _ = Command::new("powershell")
                .args(["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-File", temp_dir.to_str().unwrap(), "-TargetPid", &pid.to_string()])
                .spawn();
        }
    }
}

/// Stop hotspot
#[tauri::command]
pub fn stop_hotspot() -> Result<(), String> {
    let output = Command::new("netsh")
        .args(["wlan", "stop", "hostednetwork"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!("Failed to stop hosted network: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    logging::log_info("Hotspot stopped.");
    Ok(())
}
