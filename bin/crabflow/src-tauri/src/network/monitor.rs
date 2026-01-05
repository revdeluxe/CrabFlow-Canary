// src-tauri/src/network/monitor.rs
use serde::{Serialize, Deserialize};
use std::process::Command;
use crate::sysmodules::logging;
use tauri::State;
use std::sync::Mutex;
use sysinfo::{System, Networks};
use std::time::{SystemTime, UNIX_EPOCH};
use get_if_addrs::get_if_addrs;
use std::collections::HashMap;
use std::net::UdpSocket;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemStatus {
    pub cpu_usage: f32, // Mocked or retrieved via command
    pub memory_usage: f32, // Mocked or retrieved via command
    pub total_memory: u64, // Added for frontend calc
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_percentage: f32,
    pub app_cpu_usage: f32,
    pub app_memory_usage: u64, // Bytes
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TrafficSummary {
    pub bps_rx: u64,
    pub bps_tx: u64,
    pub tcp_pct: u8,
    pub udp_pct: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub ips: Vec<String>,
    pub mac: String,
    pub is_primary: bool,
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

/// Get system status logic
pub fn get_system_status_impl(sys: &mut System) -> SystemStatus {
    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_processes();
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_memory = sys.total_memory();
    let memory_usage = (sys.used_memory() as f64 / total_memory as f64 * 100.0) as f32;
    
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();
    let swap_percentage = if swap_total > 0 {
        (swap_used as f64 / swap_total as f64 * 100.0) as f32
    } else {
        0.0
    };

    let pid = sysinfo::Pid::from(std::process::id() as usize);
    let (app_cpu_usage, app_memory_usage) = if let Some(process) = sys.process(pid) {
        (process.cpu_usage(), process.memory())
    } else {
        (0.0, 0)
    };

    let connected = check_connection_quality();
    
    let active_interface = "eth0".to_string(); 

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    SystemStatus {
        cpu_usage,
        memory_usage,
        total_memory,
        swap_total,
        swap_used,
        swap_percentage,
        app_cpu_usage,
        app_memory_usage,
        internet_connected: connected,
        active_interface,
        timestamp,
    }
}

/// Get system status
#[tauri::command]
pub fn get_system_status(state: State<Mutex<System>>) -> SystemStatus {
    let mut sys = state.lock().unwrap();
    get_system_status_impl(&mut sys)
}

#[tauri::command]
pub fn get_traffic_summary(state: State<Mutex<Networks>>) -> TrafficSummary {
    let mut networks = state.lock().unwrap();
    networks.refresh();
    
    let mut total_rx = 0;
    let mut total_tx = 0;
    
    for (_interface_name, data) in networks.iter() {
        total_rx += data.received();
        total_tx += data.transmitted();
    }
    
    // Convert bytes to bits
    let bps_rx = total_rx * 8;
    let bps_tx = total_tx * 8;
    
    TrafficSummary {
        bps_rx,
        bps_tx,
        tcp_pct: 60, // Mocked
        udp_pct: 40, // Mocked
    }
}

#[tauri::command]
pub fn list_interfaces(state: State<Mutex<Networks>>) -> Vec<NetworkInterface> {
    let mut networks = state.lock().unwrap();
    networks.refresh_list();
    
    let mut interfaces_map: HashMap<String, NetworkInterface> = HashMap::new();

    // Detect primary IP used for internet access
    let primary_ip = UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("8.8.8.8:80")?;
            s.local_addr()
        })
        .map(|addr| addr.ip().to_string())
        .unwrap_or_default();

    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            let name = iface.name;
            let ip = iface.addr.ip().to_string();
            
            interfaces_map.entry(name.clone())
                .and_modify(|i| {
                    i.ips.push(ip.clone());
                    if ip == primary_ip {
                        i.is_primary = true;
                    }
                })
                .or_insert(NetworkInterface {
                    name,
                    ips: vec![ip.clone()],
                    mac: String::new(),
                    is_primary: ip == primary_ip,
                });
        }
    }
    
    // Enrich with MAC from sysinfo
    for (name, data) in networks.iter() {
        if let Some(iface) = interfaces_map.get_mut(name) {
            iface.mac = data.mac_address().to_string();
        }
    }
    
    interfaces_map.into_values().collect()
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
