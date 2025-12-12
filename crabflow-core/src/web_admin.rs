// src/web_admin.rs
// Specialized routes for admin-level tasks.

use rocket::{Route, routes};

// Handler for the main admin dashboard page (mounted at /admin/)
#[rocket::get("/")]
fn admin_dashboard() -> &'static str {
    "Welcome to the CrabFlow Admin Dashboard! (Requires Authentication)"
}

// Handler for a specific admin task (e.g., managing roles)
#[rocket::get("/roles")]
fn manage_roles() -> &'static str {
    "Admin: Manage User Roles and Permissions."
}

// Function to get all admin routes for mounting in main.rs
pub fn routes() -> Vec<Route> {
    routes![
        admin_dashboard,
        manage_roles,
        // Add more admin routes here (e.g., /config, /logs)
    ]
}