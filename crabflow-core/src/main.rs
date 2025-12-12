// src/main.rs

// --- MODULE DECLARATIONS ---
// These must be at the top so Rust knows the files exist.
// src/main.rs (Top section)

mod client_list;
mod router;
mod user_list;
mod filter;
mod monitor;
mod network_manager;
mod role_manager;
mod tauri_interface;
mod web_admin;
mod webpage;
mod routes; // <--- ADD THIS LINE

// ... rest of imports

// --- IMPORTS ---
use rocket::fairing::{self, AdHoc};
use rocket::{Build, routes};
use tauri::{Manager, State}; 

// --- Update rocket_builder to include ClientList ---
async fn rocket_builder(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    
    // --- Initialize Core Managers ---
    let role_manager = role_manager::RoleManager::new();
    let user_list = user_list::UserList::new(&role_manager);
    let client_list = client_list::ClientList::new(); // <--- ADDED!

    // ... (rest of the print statements)
    
    rocket
        .manage(role_manager) 
        .manage(user_list)
        .manage(client_list) // <--- ADDED!
        .mount("/", webpage::routes())
        .mount("/admin", web_admin::routes())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // We will use two threads/tasks for the concurrent Rocket and Tauri apps.

    // 1. Start the Rocket Web Server (for Web UI testing)
    let rocket_task = tokio::spawn(async {
        let _ = rocket::build()
            .attach(AdHoc::on_ignite("Manager Initialization", rocket_builder))
            .launch()
            .await;
    });

    // 2. Start the Tauri Desktop Application (for Desktop UI testing)
    // NOTE: Managers need to be rebuilt or cloned for the Tauri side if they are not in a Arc<Mutex<...>>
    let tauri_task = tokio::spawn(async {
        // Re-initialize managers for Tauri's state pool
        let tauri_role_manager = role_manager::RoleManager::new();
        let tauri_user_list = user_list::UserList::new(&tauri_role_manager);
        let tauri_client_list = client_list::ClientList::new();

        tauri::Builder::default()
            // Attach the state managers to Tauri
            .manage(tauri_role_manager)
            .manage(tauri_user_list)
            .manage(tauri_client_list)
            // Register the commands
            .invoke_handler(tauri::generate_handler![
                tauri_interface::get_monitored_clients,
                tauri_interface::check_desktop_login
                // Add all your tauri_interface commands here
            ])
            .run(tauri::generate_context!())
            .expect("Error while running Tauri application.");
    });
    
    // Wait for both tasks to complete (or one to error)
    tokio::select! {
        _ = rocket_task => {},
        _ = tauri_task => {},
    }

    Ok(())
}