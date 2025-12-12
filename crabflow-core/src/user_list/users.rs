// src/user_list/user.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
    pub role_name: String,
    pub full_name: Option<String>,
}

impl User {
    // Placeholder for password verification
    pub fn verify_password(&self, _attempt: &str) -> bool {
        true 
    }
}