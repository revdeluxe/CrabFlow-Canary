// src/role_manager.rs
// Manages roles, permissions, and access control for users and clients.

use std::collections::HashMap;

// --- 1. Permissions Enum ---
// Defines all distinct actions that can be controlled in the application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    // Client Management
    ViewClientList,
    ModifyClientConfig,
    // User Management
    ViewUserList,
    CreateUser,
    ModifyUserRoles,
    // Core Application Management
    ViewSystemMonitor,
    RestartService,
    // Web Admin Access
    AccessWebAdminDashboard,
}

// --- 2. Role Struct ---
// Defines a set of permissions associated with a human-readable role.
#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: Vec<Permission>,
}

// --- 3. RoleManager State ---
// Holds the application's configuration of roles and provides lookup methods.
#[derive(Debug)]
pub struct RoleManager {
    // Map of role names to their corresponding Role structure
    roles: HashMap<String, Role>,
}
// src/role_manager.rs (continued)

impl RoleManager {
    // Constructor: Initializes the RoleManager with a set of default roles.
    pub fn new() -> Self {
        let mut roles = HashMap::new();

        // Define a super-user role
        let admin_role = Role {
            name: "Administrator".to_string(),
            permissions: vec![
                Permission::ViewClientList,
                Permission::ModifyClientConfig,
                Permission::ViewUserList,
                Permission::CreateUser,
                Permission::ModifyUserRoles,
                Permission::ViewSystemMonitor,
                Permission::RestartService,
                Permission::AccessWebAdminDashboard,
            ],
        };
        roles.insert(admin_role.name.clone(), admin_role);

        // Define a read-only user role
        let viewer_role = Role {
            name: "Viewer".to_string(),
            permissions: vec![
                Permission::ViewClientList,
                Permission::ViewUserList,
                Permission::ViewSystemMonitor,
            ],
        };
        roles.insert(viewer_role.name.clone(), viewer_role);

        // Define a default role for unauthenticated users (if applicable)
        let guest_role = Role {
            name: "Guest".to_string(),
            permissions: vec![],
        };
        roles.insert(guest_role.name.clone(), guest_role);

        RoleManager { roles }
    }

    /// Checks if a user, identified by their role name, has a specific permission.
    pub fn check_permission(&self, role_name: &str, required_permission: &Permission) -> bool {
        match self.roles.get(role_name) {
            Some(role) => {
                role.permissions.contains(required_permission)
            }
            None => {
                // If the role doesn't exist, assume no permissions
                false
            }
        }
    }

    /// Retrieves a reference to a role by its name.
    pub fn get_role(&self, name: &str) -> Option<&Role> {
        self.roles.get(name)
    }
}

// Basic Unit Test Example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_permissions() {
        let manager = RoleManager::new();
        assert!(manager.check_permission("Administrator", &Permission::RestartService));
        assert!(manager.check_permission("Administrator", &Permission::AccessWebAdminDashboard));
    }

    #[test]
    fn test_viewer_permissions() {
        let manager = RoleManager::new();
        assert!(manager.check_permission("Viewer", &Permission::ViewClientList));
        assert!(!manager.check_permission("Viewer", &Permission::ModifyUserRoles));
    }
}