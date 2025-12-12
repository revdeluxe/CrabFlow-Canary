// src/tauri_interface.rs
// Defines the Tauri commands for desktop frontend interaction.

use tauri::{State, Window};
use crate::client_list::ClientList;
use crate::user_list::UserList;

// --- Tauri Commands ---

/// Tauri command to get the list of all monitored clients.
/// The `State` guards allow access to application-wide state managers.
#[tauri::command]
pub fn get_monitored_clients(client_list: State<'_, ClientList>) -> Result<Vec<crate::client_list::client::Client>, String> {
    Ok(client_list.get_all_clients())
}

/// Tauri command to check the login credentials from the desktop UI.
#[tauri::command]
pub fn check_desktop_login(
    username: String,
    password: String,
    user_list: State<'_, UserList>
) -> Result<String, String> {
    match user_list.authenticate_user(&username, &password) {
        Some(user) => {
            // In a real app, you would generate a session token here.
            Ok(format!("Login successful! Welcome, {} ({}).", user.full_name.as_deref().unwrap_or("User"), user.role_name))
        }
        None => {
            Err("Authentication failed. Invalid username or password.".to_string())
        }
    }
}

// NOTE: You would add more commands here, such as:
// - `send_notification_to_client(id)`
// - `get_system_status()` from the monitor module
// - `update_user_role(...)` from the role_manager module