pub mod commands;
pub mod user_management;
pub mod sysmodules;
pub mod network;

// use std::path::PathBuf;
use user_management::user::UserStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let user_db_path = sysmodules::config::get_project_root().join("db/users.json");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(UserStore::new())
        .manage(std::sync::Mutex::new(sysinfo::System::new_all()))
        .manage(std::sync::Mutex::new(sysinfo::Networks::new_with_refreshed_list()))
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            user_management::auth::login,
            user_management::auth::register_user,
            user_management::user::list_users,
            user_management::user::update_user_status,
            user_management::user::update_user_groups,
            user_management::user::get_user_settings,
            user_management::user::set_user_settings,
            network::client::list_leases,
            network::client::add_static_lease,
            network::client::remove_lease,
            network::client::list_records,
            network::client::add_record,
            network::client::remove_record,
            network::client::log_action,
            network::client::fetch_config,
            network::client::save_config,
            network::client::list_devices,
            network::monitor::get_system_status,
            network::monitor::get_traffic_summary,
            network::monitor::list_interfaces,
            network::monitor::start_wlan_monitoring
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
