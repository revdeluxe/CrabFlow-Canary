pub mod commands;
pub mod user_management;
pub mod sysmodules;

// use std::path::PathBuf;
use user_management::user::UserStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let user_db_path = sysmodules::config::get_project_root().join("db/users.json");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(UserStore::new(user_db_path))
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            user_management::auth::login,
            user_management::auth::register_user,
            user_management::user::list_users,
            user_management::user::update_user_status,
            user_management::user::update_user_groups,
            user_management::user::get_user_settings,
            user_management::user::set_user_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
