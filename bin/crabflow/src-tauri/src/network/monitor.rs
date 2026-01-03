// src-tauri/src/network/monitor.rs
use serde::{Serialize, Deserialize};
use std::process::Command;
use crate::sysmodules::logging;
use tauri::State;
use std::sync::Mutex;
use sysinfo::System;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemStatus {
    pub cpu_usage: f32, // Mocked or retrieved via command
    pub memory_usage: f32, // Mocked or retrieved via command
    pub internet_connected: bool,
    pub active_interface: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkStats {
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

/// Check internet connection quality (simple ping)
pub fn check_connection_quality() -> bool {
    // Windows ping: -n 1
    let output = Command::new("ping")
        .args(["-n", "1", "8.8.8.8"])
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

/// Get system status
#[tauri::command]
pub fn get_system_status(state: State<Mutex<System>>) -> SystemStatus {
    let mut sys = state.lock().unwrap();
    sys.refresh_cpu();
    sys.refresh_memory();
    // sys.refresh_networks(); // Not available in all versions or requires specific trait
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_usage = (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0) as f32;
    
    let connected = check_connection_quality();
    
    // Simplified interface detection for now to avoid trait issues
    let active_interface = "eth0".to_string(); 
    /*
    let active_interface = sys.networks().iter()
        .find(|(name, data)| *name != "lo" && (data.received() > 0 || data.transmitted() > 0))
        .map(|(name, _)| name.clone())
        .unwrap_or_else(|| "eth0".to_string());
    */

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    SystemStatus {
        cpu_usage,
        memory_usage,
        internet_connected: connected,
        active_interface,
        timestamp,
    }
}

/// Log system status to DB
pub fn log_system_status() {
    // let status = get_system_status(); // Requires state, cannot call easily from here without passing state
    // Placeholder

    // For now, let's assume we update post.rs or use a specific name.
    
    // Let's use a file that we will route to DB in post.rs
    // let _ = post::append_file("system_stats.json", &format!("{}\n", json));
}

/// Start WLAN monitoring (Promiscuous/Monitor mode simulation)
#[tauri::command]
pub fn start_wlan_monitoring(interface: String) {
    logging::log_info(&format!("Starting WLAN monitoring on {}", interface));
    // Real implementation requires pcap/AirPcap on Windows
    // For now, we list visible networks
    let output = Command::new("netsh")
        .args(["wlan", "show", "networks", "mode=bssid"])
        .output();
        
    if let Ok(o) = output {
        let result = String::from_utf8_lossy(&o.stdout);
        logging::log_debug(&format!("WLAN Scan Results:\n{}", result));
    }
}
