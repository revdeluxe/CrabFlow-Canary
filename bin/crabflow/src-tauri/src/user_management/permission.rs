use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    ManageNetwork,
    ViewLogs,
    ManageUsers,
    InternetAccess,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Staff, // Kept for backward compatibility if needed, or treated as UserManager
    UserManager,
    Guest,
}

impl Role {
    pub fn get_permissions(&self) -> Vec<Permission> {
        match self {
            Role::Admin => vec![
                Permission::ManageNetwork,
                Permission::ViewLogs,
                Permission::ManageUsers,
                Permission::InternetAccess,
            ],
            Role::Staff | Role::UserManager => vec![
                Permission::ViewLogs,
                Permission::ManageUsers,
            ],
            Role::Guest => vec![
                Permission::InternetAccess,
            ],
        }
    }
}

pub fn check_access(role: &Role, required: &Permission) -> bool {
    role.get_permissions().contains(required)
}
