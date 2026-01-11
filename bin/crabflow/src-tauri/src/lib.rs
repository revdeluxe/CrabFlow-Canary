pub mod commands;
pub mod user_management;
pub mod sysmodules;
pub mod network;

use user_management::user::UserStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _user_db_path = sysmodules::config::get_project_root().join("db/users.json");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(UserStore::new())
        .manage(std::sync::Mutex::new(sysinfo::System::new_all()))
        .manage(std::sync::Mutex::new(sysinfo::Networks::new_with_refreshed_list()))
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            
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
            network::client::list_devices,

            // Network Monitor
            network::monitor::get_system_status,
            network::monitor::get_live_stats,
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
            network::cportal::set_custom_portal,
            network::cportal::get_portal_template,
            network::cportal::save_portal_template,

            // ACL
            network::acl::get_acl_config,
            network::acl::save_acl_config,

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
