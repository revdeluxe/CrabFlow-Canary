// src/user_list/mod.rs
// Manages the collection of users and provides authentication services.

use std::collections::HashMap;
use crate::role_manager::RoleManager; // Needed to link roles
use crate::user_list::user::User; // Import the User struct

// Re-export the user struct for use by other modules
pub mod user;

/// The main structure for managing all system users.
#[derive(Debug)]
pub struct UserList {
    users: HashMap<String, User>, // Keyed by username for quick lookup
}

impl UserList {
    /// Creates a new UserList and initializes it with default users.
    pub fn new(role_manager: &RoleManager) -> Self {
        let mut users = HashMap::new();

        // 1. Create an Administrator user
        if role_manager.get_role("Administrator").is_some() {
            let admin = User {
                id: 1,
                username: "admin".to_string(),
                password_hash: "HashedAdminPassword123".to_string(), // Placeholder
                role_name: "Administrator".to_string(),
                full_name: Some("System Administrator".to_string()),
                created_at: "2023-01-01".to_string(),
                last_login_at: None,
            };
            users.insert(admin.username.clone(), admin);
        }

        // 2. Create a Viewer user
        if role_manager.get_role("Viewer").is_some() {
            let viewer = User {
                id: 2,
                username: "viewer".to_string(),
                password_hash: "HashedViewerPassword456".to_string(), // Placeholder
                role_name: "Viewer".to_string(),
                full_name: Some("Public Viewer".to_string()),
                created_at: "2023-01-01".to_string(),
                last_login_at: None,
            };
            users.insert(viewer.username.clone(), viewer);
        }

        UserList { users }
    }

    /// Attempts to find a user and verify their credentials.
    pub fn authenticate_user(&self, username: &str, password: &str) -> Option<&User> {
        self.users.get(username)
            .filter(|user| user.verify_password(password))
    }
    
    /// Retrieves a list of all users. Useful for the admin panel.
    pub fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
    
    // NOTE: In a real application, you would add methods for:
    // - AddUser, DeleteUser, UpdateRole, etc.
}