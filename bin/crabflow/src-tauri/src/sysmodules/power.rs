use std::process::Command;
use tauri::AppHandle;
use crate::network::init::{shutdown_networking, initialize_networking};
use crate::sysmodules::logging;

#[tauri::command]
pub fn shutdown_system() -> Result<(), String> {
    logging::log_info("Initiating system shutdown...");
    
    #[cfg(target_os = "windows")]
    let status = Command::new("shutdown")
        .args(&["/s", "/t", "0"])
        .status();

    #[cfg(not(target_os = "windows"))]
    let status = Command::new("shutdown")
        .args(&["-h", "now"])
        .status();

    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to execute shutdown command: {}", e)),
    }
}

#[tauri::command]
pub fn restart_system() -> Result<(), String> {
    logging::log_info("Initiating system restart...");

    #[cfg(target_os = "windows")]
    let status = Command::new("shutdown")
        .args(&["/r", "/t", "0"])
        .status();

    #[cfg(not(target_os = "windows"))]
    let status = Command::new("reboot")
        .status();

    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to execute restart command: {}", e)),
    }
}

#[tauri::command]
pub fn restart_networking() -> Result<(), String> {
    logging::log_info("Restarting networking services...");
    shutdown_networking();
    // Give it a moment to clear up ports if necessary, though shutdown should be synchronous enough
    std::thread::sleep(std::time::Duration::from_secs(1));
    initialize_networking();
    Ok(())
}

#[tauri::command]
pub fn restart_application(app: AppHandle) {
    logging::log_info("Restarting application...");
    app.restart();
}
