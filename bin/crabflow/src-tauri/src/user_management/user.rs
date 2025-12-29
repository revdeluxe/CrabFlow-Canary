// src-tauri/src/user.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub role: String,
    pub groups: Vec<String>,
}

#[tauri::command]
pub fn list_users() -> Vec<User> {
    vec![
        User { id: "1".into(), username: "admin".into(), role: "admin".into(), groups: vec!["staff".into()] },
        User { id: "2".into(), username: "student1".into(), role: "student".into(), groups: vec!["engineering".into()] },
    ]
}

#[tauri::command]
pub fn set_user_groups(user_id: String, groups: Vec<String>) -> Result<(), String> {
    // For now, just log or stub
    println!("Set groups for {} -> {:?}", user_id, groups);
    Ok(())
}

#[tauri::command]
pub fn filter_users_by_group(group: String) -> Vec<User> {
    list_users().into_iter().filter(|u| u.groups.contains(&group)).collect()
}
