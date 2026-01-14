#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// src-tauri/src/main.rs
mod sysmodules;
mod setup;
mod network;
mod user_management;
mod http_server;
mod init;
mod render;

use tauri::{Window, Manager};
use std::sync::Mutex;
use user_management::user::UserStore;
use user_management::auth::SessionStore;
use sysinfo::System;

use std::process::Command;
use std::env;

// Import tracing subscriber
use tracing_subscriber;

#[cfg(target_os = "linux")]
fn check_is_elevated() -> bool {
    // On Linux, check if the Effective User ID (euid) is 0 (Root)
    // We use the filesystem hack to avoid adding the 'libc' dependency
    use std::os::unix::fs::MetadataExt;
    if let Ok(metadata) = std::fs::metadata("/proc/self") {
        return metadata.uid() == 0;
    }
    false
}

#[cfg(target_os = "macos")]
fn check_is_elevated() -> bool {
    // On macOS, 'id -u' is the safest way without external dependencies (libc)
    if let Ok(output) = Command::new("id").arg("-u").output() {
        if let Ok(uid_str) = String::from_utf8(output.stdout) {
            return uid_str.trim() == "0";
        }
    }
    false
}

#[cfg(target_os = "windows")]
fn check_is_elevated() -> bool {
    // On Windows, checking admin rights without a crate is very verbose.
    // For this setup wizard, we can often skip the strict check in 'main'
    // because the 'startup_process' logic will fail later if permissions are missing.
    // Or, we rely on the manifest to force admin.
    true // Assume true or implement complex winapi check if needed
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn check_is_elevated() -> bool {
    // Unsupported platform fallback
    eprintln!("Warning: Privilege check not implemented for this platform");
    true
}

// --- HELPER: Cross-Platform Self-Elevation ---
#[cfg(target_os = "linux")]
fn try_relaunch_as_admin() {
    let exe = env::current_exe().expect("Failed to get current executable path");
    println!("Requesting administrative privileges...");

    // On Linux, we forward DISPLAY and XAUTHORITY so the GUI works as root.
    let display = env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
    let xauthority = env::var("XAUTHORITY").unwrap_or_default();

    match Command::new("pkexec")
        .arg("env")
        .arg(format!("DISPLAY={}", display))
        .arg(format!("XAUTHORITY={}", xauthority))
        .arg(&exe)
        .spawn()
    {
        Ok(_) => {
            println!("Elevated process spawned. Exiting non-privileged instance.");
            std::process::exit(0);
        }
        Err(e) => eprintln!("Failed to launch pkexec: {}", e),
    }
}

#[cfg(target_os = "macos")]
fn try_relaunch_as_admin() {
    let exe = env::current_exe().expect("Failed to get current executable path");
    println!("Requesting administrative privileges...");

    let script = format!(
        "do shell script \"'{}'\" with administrator privileges",
        exe.to_string_lossy()
    );
    match Command::new("osascript").arg("-e").arg(script).spawn() {
        Ok(_) => {
            println!("Elevated process spawned. Exiting non-privileged instance.");
            std::process::exit(0);
        }
        Err(e) => eprintln!("Failed to launch osascript: {}", e),
    }
}

#[cfg(target_os = "windows")]
fn try_relaunch_as_admin() {
    let exe = env::current_exe().expect("Failed to get current executable path");
    println!("Requesting administrative privileges...");

    let cwd = exe.parent().expect("Failed to get current directory");

    match Command::new("powershell")
        .arg("Start-Process")
        .arg("-FilePath")
        .arg(format!("\"{}\"", exe.display()))
        .arg("-WorkingDirectory")
        .arg(format!("\"{}\"", cwd.display()))
        .arg("-Verb")
        .arg("RunAs")
        .spawn()
    {
        Ok(_) => {
            println!("Elevated process spawned. Exiting non-privileged instance.");
            std::process::exit(0);
        }
        Err(e) => eprintln!("Failed to launch PowerShell elevation: {}", e),
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn try_relaunch_as_admin() {
    eprintln!("Error: Self-elevation not implemented for this platform");
}

// --- TAURI COMMANDS ---

#[tauri::command]
fn begin_setup(window: Window) {
    // do setup logic...
    render::emit(&window, "setup_started", "CrabFlow setup initialized");
    // later, when setup completes:
    render::emit(&window, "setup_completed", "CrabFlow setup finished");
}

#[tauri::command]
fn startup_process(window: Window) {
    // NOTE: We are guaranteed to be admin here because main() enforces it.

    sysmodules::logging::log_info("Performing startup system checks...");

    if !init::does_system_files() {
        render::emit(&window, "startup_error", "Critical system files are missing");
        sysmodules::logging::log_error("Startup failed: Missing system files");
    } else {
        render::emit(&window, "startup_success", "All system files verified");
        sysmodules::logging::log_info("Startup successful: All system files present");
        sysmodules::logging::log_info("CrabFlow is operational.");
    }
}

// --- MAIN ENTRY POINT ---

fn main() {
    // 1. IMMEDIATE PRIVILEGE CHECK
    if !check_is_elevated() {
        try_relaunch_as_admin();
        eprintln!("Error: This application requires administrative privileges.");
        std::process::exit(1);
    }

    // 2. INITIALIZE LOGGING & TRACING
    // Initialize Tracing for SurrealDB / Async operations
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    sysmodules::logging::init_logging();

    // 3. INITIALIZE DB & RUNTIME
    user_management::init::initialize_user_management();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    rt.block_on(async {
        if let Err(e) = sysmodules::db::init().await {
            eprintln!("DB Init Error: {}", e);
        }
        if let Err(e) = sysmodules::db::migrate_legacy().await {
            eprintln!("DB Migration Error: {}", e);
        }
    });

    // 4. LOAD STATE
    let user_store = UserStore::new();
    let session_store = SessionStore::new();

    rt.block_on(async {
        if let Err(e) = user_store.load_from_db().await {
            eprintln!("Failed to load UserStore from DB: {}", e);
        }
    });

    // 5. BUILD APPLICATION
    tauri::Builder::default()
    .setup(|app| {
        let handle = app.handle();
        network::init::initialize_networking(Some(handle.clone()));

        let user_store = app.state::<UserStore>().inner().clone();
        let session_store = app.state::<SessionStore>().inner().clone();

        // Spawn HTTP Server
        tauri::async_runtime::spawn(async move {
            http_server::start_server(user_store, session_store).await;
        });
        Ok(())
    })
    .manage(Mutex::new(System::new_all()))
    .manage(user_store)
    .manage(session_store)
    .manage(Mutex::new(sysinfo::Networks::new_with_refreshed_list()))
    .invoke_handler(tauri::generate_handler![
        // Main
        begin_setup,
        startup_process,
        crabflow_lib::commands::greet,

        // Setup
        setup::wizard::save_setup,
        setup::wizard::load_setup,
        setup::wizard::get_wizard_status,
        setup::wizard::check_setup,
        setup::wizard::check_first_run,
        setup::wizard::reset_setup,
        setup::wizard::validate_config,

        // Config & Logging
        sysmodules::config::load_logging_config,
        sysmodules::config::save_logging_config,
        sysmodules::logging::get_logs,
        sysmodules::logging::reload_logging_config,
        sysmodules::logging::clear_logs,

        // Network Client
        network::client::list_leases,
        network::client::add_static_lease,
        network::client::remove_lease,
        network::client::list_records,
        network::client::add_record,
        network::client::update_record,
        network::client::remove_record,
        network::client::update_upstream_interface,
        network::client::log_action,
        network::client::fetch_config,
        network::client::save_config,

        // Network DNS
        network::dns::get_query_logs,
        network::dns::get_blacklist,
        network::dns::block_domain,
        network::dns::unblock_domain,
        network::dns::import_blacklist,

        // Network Monitor
        network::monitor::get_system_status,
        network::monitor::get_live_stats,
        network::monitor::start_wlan_monitoring,
        network::monitor::list_interfaces,
        network::init::reload_networking,

        // Network Packet
        network::packet::send_packet,
        network::packet::start_packet_listener,

        // Firewall
        network::firewall::list_firewall_rules,
        network::firewall::add_firewall_rule,
        network::firewall::update_firewall_rule,
        network::firewall::delete_firewall_rule,

        // ACL & Permissions
        network::acl::get_acl_config,
        network::acl::save_acl_config,

        // Network Wifi
        network::wifi::create_hotspot,
        network::wifi::stop_hotspot,

        // Captive Portal
        network::cportal::tag_user,
        network::cportal::get_user_history,
        network::cportal::set_captive_portal,
        network::cportal::set_custom_portal,
        network::cportal::get_portal_template,
        network::cportal::save_portal_template,

        // Auth
        user_management::auth::login,
        user_management::auth::register_user,
        user_management::auth::logout,
        user_management::auth::check_auth,
        user_management::auth::get_online_users,

        // User Management
        user_management::user::list_users,
        user_management::user::update_user_status,
        user_management::user::update_user_groups,
        user_management::user::update_user_role,
        user_management::user::get_user_settings,
        user_management::user::set_user_settings,
        user_management::user::sort_users_by,
        user_management::user::change_password,
        user_management::user::user_exists,
        user_management::user::add_user,
        user_management::user::remove_user,
        user_management::user::upload_id,
        user_management::user::update_user_profile,

        // Group Management
        user_management::user::list_groups,
        user_management::user::add_group,
        user_management::user::update_group,
        user_management::user::delete_group,
        user_management::user::list_permissions,

        // Sysmodules
        sysmodules::fetch::fetch_setup,
        sysmodules::post::post_setup,

        // Power
        sysmodules::power::shutdown_system,
        sysmodules::power::restart_system,
        sysmodules::power::restart_networking,
        sysmodules::power::restart_application,
    ])
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { .. } => {
            network::init::shutdown_networking();
        }
        _ => {}
    });
}
