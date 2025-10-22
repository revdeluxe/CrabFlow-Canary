use gethostname::gethostname;
use getifaddrs::getifaddrs;
use serde::Serialize;
use std::collections::HashMap;
use sysinfo::{CpuExt, NetworkExt, System, SystemExt};

#[derive(Serialize)]
pub struct DashboardContext {
    pub title: String,
    pub hostname: String,
    pub os: String,
    pub cpu: String,
    pub memory: String,
    pub interfaces: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct InterfaceTraffic {
    pub name: String,
    pub rx: u64,
    pub tx: u64,
}

pub fn get_network_traffic() -> Vec<InterfaceTraffic> {
    let mut sys = System::new_all();
    sys.refresh_networks();
    let networks = sys.networks();
    let mut traffic_data = Vec::new();
    for (iface_name, data) in networks {
        traffic_data.push(InterfaceTraffic {
            name: iface_name.clone(),
            rx: data.received(),
            tx: data.transmitted(),
        });
    }
    traffic_data
}

pub fn gather_dashboard_context() -> DashboardContext {
    // sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    // Hostname
    let hostname = gethostname().to_string_lossy().into_owned();

    // OS: fall back to simple platform name to avoid sysinfo version method differences
    let os = std::env::consts::OS.to_string();

    // CPU: use first CPU brand if present
    let cpu = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".into());

    // Memory in MB
    let memory = format!("{} MB", sys.total_memory() / 1024);

    // Interfaces
    let mut interface_map: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(ifaces) = getifaddrs() {
        for iface in ifaces {
            let ip_str = match iface.address {
                getifaddrs::Address::V4(addr) => Some(addr.address.to_string()),
                getifaddrs::Address::V6(addr) => Some(addr.address.to_string()),
                _ => None,
            };
            if let Some(ip) = ip_str {
                interface_map.entry(iface.name).or_default().push(ip);
            }
        }
    }

    let mut interfaces: Vec<String> = interface_map
        .into_iter()
        .map(|(name, ips)| format!("{} → {}", name, ips.join(", ")))
        .collect();
    interfaces.sort();

    DashboardContext {
        title: "CrabFlow Canary".into(),
        hostname,
        os,
        cpu,
        memory,
        interfaces,
    }
}
