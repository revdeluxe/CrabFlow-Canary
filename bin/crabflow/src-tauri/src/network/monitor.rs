// src-tauri/src/network/monitor.rs
use serde::{Serialize, Deserialize};
use std::process::Command;
use crate::sysmodules::logging;
use crate::network::{dhcp, dns};
use crate::user_management::auth::SessionStore;
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

use std::sync::atomic::{AtomicBool, Ordering};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

static LOGGING_ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveServiceStats {
    pub dhcp_clients: usize,
    pub dns_queries_total: usize,
    pub active_users: usize,
    pub services_status: HashMap<String, bool>,
}

#[tauri::command]
pub fn get_live_stats(session_store: State<SessionStore>) -> LiveServiceStats {
    let dhcp_clients = dhcp::list_leases().len();
    let dns_queries_total = dns::get_query_count();
    
    let mut services = HashMap::new();
    services.insert("dhcp".to_string(), dhcp::is_server_running());
    services.insert("dns".to_string(), dns::is_server_running());
    
    let active_users = match session_store.sessions.lock() {
        Ok(sessions) => sessions.len(),
        Err(_) => 0,
    };

    LiveServiceStats {
        dhcp_clients,
        dns_queries_total,
        active_users,
        services_status: services,
    }
}

#[tauri::command]
pub fn start_performance_logging(filename: String, _state_sys: State<'static, Mutex<System>>, _state_net: State<'static, Mutex<Networks>>) {
    if LOGGING_ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    LOGGING_ACTIVE.store(true, Ordering::Relaxed);
    
    // We need to clone the states to pass them to the thread
    // Wait, State is essentially an wrapper around Arc<T>, but we can't clone State directly in way that works easily in thread spawn unless we use inner().
    // However, the State<T> passed here argument is defined in the command.
    // We can't pass 'state' directly to thread. 
    // We need Arc<Mutex<System>>. State implements Deref, but to move to thread we need the Arc.
    // state.inner() gives &T. 
    // Actually, in Tauri v1/v2, State<T> can be cloned if we specifically use the right type.
    // The main.rs manages `Mutex<System>`. So State<Mutex<System>> is actually holding a reference? No it holds Arc.
    // `state.inner()` returns `&Mutex<System>`. 
    // We need the `Arc` to move it. 
    // But we cannot get the Arc back from `State`. 
    // Wait, in previous edits we changed main.rs to manage `Arc<Mutex<System>>`?
    // Let's check main.rs again. `manage(Mutex::new(System::new_all()))`.
    // So Tauri holds `Arc<Mutex<System>>` internally, but exposed as `State<Mutex<System>>`.
    // We cannot clone the Arc from the State.
    // 
    // Workaround: We can't access Tauri state inside a detached thread easily unless we pass the Arc ourselves.
    // BUT we can't get the Arc. 
    //
    // Actually, the user asked to "spawn a background thread". 
    // If we can't share the existing State `System`, we might need to create a new `System` instance?
    // "get_system_status_impl" takes `&mut System`.
    // 
    // Let's look at `http_server.rs`. It holds `Arc<Mutex<System>>`.
    // In `main.rs`, we do `.manage(Mutex::new(..))`.
    //
    // If we want to use the same `System` object (to avoid overhead), we should probably Wrap it in Arc before passing to manage.
    // `main.rs`: `.manage(Arc::new(Mutex::new(System::new_all())))`.  <-- This would change the type to `State<Arc<Mutex<System>>>`.
    //
    // Let's assume for this task, creating a *new* System instance locally in the thread is acceptable, 
    // OR we change main.rs to manage Arc.
    // Given the prompt "get_system_status_impl ... to get current stats", it implies using the main system struct.
    // 
    // Let's try to assume we can just create a `System` inside the thread for logging. 
    // It's cleaner than refactoring the whole app state type.
    // EXCEPT `get_traffic_summary` uses `Networks`.
    
    thread::spawn(move || {
        let mut sys = System::new_all();
        let mut networks = Networks::new_with_refreshed_list();
        
        // Open file
        let mut file = match OpenOptions::new().create(true).append(true).open(&filename) {
            Ok(f) => f,
            Err(e) => {
                logging::log_error(&format!("Failed to open log file: {}", e));
                return;
            }
        };

        // Write header if empty
        if let Ok(metadata) = file.metadata() {
            if metadata.len() == 0 {
                if let Err(_) = writeln!(file, "timestamp,cpu_usage,memory_percentage,total_bytes_rx,total_bytes_tx") {
                     logging::log_error("Failed to write CSV header");
                }
            }
        }

        while LOGGING_ACTIVE.load(Ordering::Relaxed) {
             // Refresh & Get Data
             sys.refresh_all();
             networks.refresh_list();
             networks.refresh();

             // CPU
             let cpu_usage = sys.global_cpu_info().cpu_usage();
             
             // Memory
             let total_mem = sys.total_memory();
             let used_mem = sys.used_memory();
             let mem_pct = if total_mem > 0 {
                 (used_mem as f32 / total_mem as f32) * 100.0
             } else {
                 0.0
             };

             // Traffic
             let mut total_rx = 0;
             let mut total_tx = 0;
             for (_name, data) in &networks {
                 total_rx += data.total_received();
                 total_tx += data.total_transmitted();
             }

             let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

             if let Err(e) = writeln!(file, "{},{},{},{},{}", ts, cpu_usage, mem_pct, total_rx, total_tx) {
                 logging::log_error(&format!("Failed to write log entry: {}", e));
             }

             thread::sleep(Duration::from_secs(1));
        }
    });
}

#[tauri::command]
pub fn stop_performance_logging() {
    LOGGING_ACTIVE.store(false, Ordering::Relaxed);
}

/// Check internet connection quality (simple ping)
pub fn check_connection_quality() -> bool {
    #[cfg(target_os = "windows")]
    let args = ["-n", "1", "-w", "1000", "8.8.8.8"];
    
    #[cfg(not(target_os = "windows"))]
    let args = ["-c", "1", "-W", "1", "8.8.8.8"];
    
    let output = Command::new("ping")
        .args(args)
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
    
    #[cfg(target_os = "windows")]
    {
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
    
    #[cfg(target_os = "linux")]
    {
        // On Linux, use iwlist or iw to scan
        let output = Command::new("iwlist")
            .args([&interface, "scan"])
            .output();
            
        if let Ok(o) = output {
            let result = String::from_utf8_lossy(&o.stdout);
            logging::log_debug(&format!("WLAN Scan Results:\n{}", result));
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // On macOS, use airport utility
        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .args(["-s"])
            .output();
            
        if let Ok(o) = output {
            let result = String::from_utf8_lossy(&o.stdout);
            logging::log_debug(&format!("WLAN Scan Results:\n{}", result));
        }
    }
}
