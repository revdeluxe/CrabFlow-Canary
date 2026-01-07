#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// src-tauri/src/main.rs
// reserved for main application logic and user interaction flow
mod sysmodules;
mod setup;
mod network;
mod user_management;
mod http_server; // New module

mod init;
mod render;

use tauri::{Window, Manager};
// use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use user_management::user::UserStore;
use user_management::auth::SessionStore;
use sysinfo::System;
use is_elevated::is_elevated;

#[tauri::command]
fn begin_setup(window: Window) {
    // do setup logic...
    render::emit(&window, "setup_started", "CrabFlow setup initialized");
    // later, when setup completes:
    render::emit(&window, "setup_completed", "CrabFlow setup finished");
}

#[tauri::command]
fn startup_process(window: Window) {
    // Check for admin privileges
    if !is_elevated::is_elevated() {
        sysmodules::logging::log_info("Requesting admin privileges...");
        
        if let Ok(exe) = std::env::current_exe() {
            let cwd = exe.parent().unwrap();
            // Relaunch as admin using PowerShell
            // We use "Start-Process -Verb RunAs" to trigger UAC
            let _ = std::process::Command::new("powershell")
                .arg("Start-Process")
                .arg("-FilePath")
                .arg(format!("\"{}\"", exe.display())) // Quote the path to handle spaces
                .arg("-WorkingDirectory")
                .arg(format!("\"{}\"", cwd.display()))
                .arg("-Verb")
                .arg("RunAs")
                .spawn();
            
            // Exit the current non-admin process
            std::process::exit(0);
        }
    }

    if !init::does_system_files() {
        render::emit(&window, "startup_error", "Critical system files are missing");
        sysmodules::logging::log_error("Startup failed: Missing system files");
    } else {
        render::emit(&window, "startup_success", "All system files verified"); 
        sysmodules::logging::log_info("Startup successful: All system files present");
        sysmodules::logging::log_info("CrabFlow is operational, S"); 
    }

}

fn main() {
    if !is_elevated() {
        eprintln!("Error: This application requires administrative privileges. Please run as Administrator or root.");
        
        #[cfg(windows)]
        {
            // Optional: You could try to self-elevate here via ShellExecute, 
            // but the manifest should prevent us getting here on Windows.
        }
        std::process::exit(1);
    }

    dotenv::dotenv().ok();
    sysmodules::logging::init_logging(); 
    network::init::initialize_networking(); 
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

    // Create shared state
    let user_store = UserStore::new();
    let session_store = SessionStore::new();
    
    rt.block_on(async {
         if let Err(e) = user_store.load_from_db().await {
             eprintln!("Failed to load UserStore from DB: {}", e);
         }
    });

    tauri::Builder::default()
        .setup(|app| {
            let user_store = app.state::<UserStore>().inner().clone();
            let session_store = app.state::<SessionStore>().inner().clone();
            
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
            network::dns::get_query_logs,
            network::dns::get_blacklist,
            network::dns::block_domain,
            network::dns::unblock_domain,
            network::dns::import_blacklist,
            network::client::log_action,
            network::client::fetch_config,
            network::client::save_config,

            // Network Monitor
            network::monitor::get_system_status,
            network::monitor::start_wlan_monitoring,
            network::monitor::list_interfaces,

            // Network Packet
            network::packet::send_packet,
            network::packet::start_packet_listener,

            // Firewall
            network::firewall::list_firewall_rules,
            network::firewall::add_firewall_rule,
            network::firewall::update_firewall_rule,
            network::firewall::delete_firewall_rule,

            // Network Wifi
            network::wifi::create_hotspot,
            network::wifi::stop_hotspot,

            // Captive Portal
            network::cportal::tag_user,
            network::cportal::get_user_history,
            network::cportal::set_captive_portal,
            network::cportal::get_portal_template,
            network::cportal::save_portal_template,

            // Auth (New)
            user_management::auth::login,
            user_management::auth::register_user,
            user_management::auth::logout,
            user_management::auth::check_auth,
            user_management::auth::get_online_users,

            // User Management
            user_management::user::list_users,
            user_management::user::update_user_status,
            user_management::user::update_user_groups,
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
            sysmodules::logging::get_logs,
            
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
