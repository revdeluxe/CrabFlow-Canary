// src-tauri/src/main.rs
// reserved for main application logic and user interaction flow
mod sysmodules;
mod setup;
mod network;
mod user_management;

mod init;
mod render;

use tauri::Window;
// use std::path::PathBuf;
use std::sync::Mutex;
use user_management::user::UserStore;
use user_management::auth::SessionStore;
use sysinfo::System;

#[tauri::command]
fn begin_setup(window: Window) {
    // do setup logic...
    render::emit(&window, "setup_started", "CrabFlow setup initialized");
    // later, when setup completes:
    render::emit(&window, "setup_completed", "CrabFlow setup finished");
}

#[tauri::command]
fn startup_process(window: Window) {
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
    dotenv::dotenv().ok();
    sysmodules::logging::init_logging(); 
    network::init::initialize_networking(); 
    user_management::init::initialize_user_management(); 
    
    let user_db_path = sysmodules::config::get_project_root().join("db/users.json");

    tauri::Builder::default()
        .manage(Mutex::new(System::new_all()))
        .manage(UserStore::new(user_db_path))
        .manage(SessionStore::new())
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

            // Network Client
            network::client::list_leases,
            network::client::add_static_lease,
            network::client::remove_lease,
            network::client::list_records,
            network::client::add_record,
            network::client::remove_record,
            network::client::log_action,
            network::client::fetch_config,
            network::client::save_config,

            // Network Monitor
            network::monitor::get_system_status,
            network::monitor::start_wlan_monitoring,

            // Network Packet
            network::packet::send_packet,
            network::packet::start_packet_listener,

            // Network Wifi
            network::wifi::create_hotspot,
            network::wifi::stop_hotspot,

            // Captive Portal
            network::cportal::tag_user,
            network::cportal::get_user_history,
            network::cportal::set_captive_portal,

            // Auth (New)
            user_management::auth::login,
            user_management::auth::register_user,
            user_management::auth::logout,
            user_management::auth::check_auth,

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
