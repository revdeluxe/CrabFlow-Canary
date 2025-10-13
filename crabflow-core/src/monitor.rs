use serde::Serialize;
use sysinfo::System;
use gethostname::gethostname;
use getifaddrs::getifaddrs;

#[derive(Serialize)]
pub struct DashboardContext {
    pub title: String,
    pub hostname: String,
    pub os: String,
    pub cpu: String,
    pub memory: String,
    pub interfaces: Vec<String>,
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

    // Interfaces: render address with debug formatter to avoid matching on internal Address variants
    let interfaces: Vec<String> = getifaddrs()
        .map(|ifaces| {
            ifaces
                .map(|iface| {
                    let ip = format!("{:?}", iface.address);
                    format!("{} → {}", iface.name, ip)
                })
                .collect()
        })
        .unwrap_or_default();

    DashboardContext {
        title: "CrabFlow Canary".into(),
        hostname,
        os,
        cpu,
        memory,
        interfaces,
    }
}
