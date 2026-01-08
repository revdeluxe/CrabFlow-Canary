use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::State;
use crate::user_management::permission::Role;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginRecord {
    pub ip: String,
    pub mac: String,
    pub timestamp: String, // ISO 8601
    pub device_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub password_hash: String, // In a real app, use bcrypt/argon2
    pub role: Role, // "admin", "user", "guest"
    pub groups: Vec<String>,
    pub is_active: bool,
    pub is_approved: bool,
    #[serde(default)]
    pub login_history: Vec<LoginRecord>,
    #[serde(default)]
    pub id_document_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSettings {
    pub auto_approve_new_users: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDatabase {
    pub users: Vec<User>,
    #[serde(default)]
    pub groups: Vec<Group>,
    pub settings: UserSettings,
}


#[derive(Clone)]
pub struct UserStore {
    pub db: Arc<Mutex<UserDatabase>>,
}

impl UserStore {
    pub fn new() -> Self {
        // Initialize with default or empty, real data loaded via load_from_db
        Self {
            db: Arc::new(Mutex::new(UserDatabase::default())),
        }
    }

    pub async fn load_from_db(&self) -> Result<(), String> {
        let db = crate::sysmodules::db::get();
        // Load Users
        let users: Vec<User> = db.select("users").await.map_err(|e| e.to_string())?;
        // Load Groups
        let groups: Vec<Group> = db.select("groups").await.map_err(|e| e.to_string())?;
        // Load Settings (Single Record)
        let settings: Option<UserSettings> = db.select(("settings", "main")).await.map_err(|e| e.to_string())?;

        let mut data = self.db.lock().map_err(|e| e.to_string())?;
        
        if !users.is_empty() {
             data.users = users.clone();
        } else {
            // Ensure default admin exists if DB is empty? 
            // Default impl of UserDatabase has admin. If DB is empty, we keep default and Save it?
            // Yes, let's allow saving defaults to DB if empty.
        }

        if !groups.is_empty() {
            data.groups = groups.clone();
        }

        if let Some(s) = settings {
            data.settings = s;
        }
        
        drop(data);
        
        // If we loaded empty but had defaults, persist them back to ensure DB is initialized
        if users.is_empty() && groups.is_empty() {
             self.persist().await?;
        }

        Ok(())
    }

    pub async fn persist(&self) -> Result<(), String> {
        let data = self.db.lock().map_err(|e| e.to_string())?.clone();
        let db = crate::sysmodules::db::get();
        
        // Update Users (Upsert)
        for user in data.users {
            let _: Option<User> = db.update(("users", &user.username)).content(user).await.ok().flatten();
        }
        
        // Update Groups
        for group in data.groups {
            let _: Option<Group> = db.update(("groups", &group.name)).content(group).await.ok().flatten();
        }
        
        // Update Settings
        let _: Option<UserSettings> = db.update(("settings", "main")).content(data.settings).await.ok().flatten();
        
        // Note: This logic inserts/updates but doesn't handle deletions efficiently 
        // (deleted from memory -> remains in DB). 
        // Proper fix requires explicit delete calls, handled in commands.
        // For 'Fully migrate', ideally we use direct DB calls everywhere, but this Hybrid approach 
        // preserves the synchronicity of read locks while supporting SurrealDB storage.
        
        Ok(())
    }
}

impl Default for UserDatabase {
    fn default() -> Self {
        Self {
            users: vec![
                User {
                    username: "admin".to_string(),
                    nickname: Some("Administrator".to_string()),
                    email: Some("admin@example.com".to_string()),
                    password_hash: "admin".to_string(), // Default password
                    role: Role::Admin,
                    groups: vec!["admin".to_string()],
                    is_active: true,
                    is_approved: true,
                    login_history: vec![],
                    id_document_path: None,
                }
            ],
            groups: vec![
                Group {
                    name: "admin".to_string(),
                    description: "Administrators with full access".to_string(),
                    permissions: vec!["*".to_string()],
                },
                Group {
                    name: "user".to_string(),
                    description: "Standard users".to_string(),
                    permissions: vec!["portal:access".to_string()],
                }
            ],
            settings: UserSettings {
                auto_approve_new_users: false,
            },
        }
    }
}

// Commands

#[tauri::command]
pub fn list_users(store: State<UserStore>) -> Result<Vec<User>, String> {
    let db = store.db.lock().map_err(|e| e.to_string())?;
    Ok(db.users.clone())
}

// Group Management Commands

#[tauri::command]
pub fn list_groups(store: State<UserStore>) -> Result<Vec<Group>, String> {
    let db = store.db.lock().map_err(|e| e.to_string())?;
    Ok(db.groups.clone())
}

#[tauri::command]
pub async fn add_group(store: State<'_, UserStore>, name: String, description: String, permissions: Vec<String>) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        if db.groups.iter().any(|g| g.name == name) {
            return Err("Group already exists".to_string());
        }
        db.groups.push(Group { name, description, permissions });
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub async fn update_group(store: State<'_, UserStore>, name: String, description: String, permissions: Vec<String>) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        if let Some(group) = db.groups.iter_mut().find(|g| g.name == name) {
            group.description = description;
            group.permissions = permissions;
        } else {
            return Err("Group not found".to_string());
        }
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_group(store: State<'_, UserStore>, name: String) -> Result<(), String> {
    if name == "admin" {
        return Err("Cannot delete admin group".to_string());
    }
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        db.groups.retain(|g| g.name != name);
    }
    store.persist().await?;
    // Explicit DB Delete
    let _: Option<Group> = crate::sysmodules::db::get().delete(("groups", &name)).await.ok().flatten();
    Ok(())
}

#[tauri::command]
pub fn list_permissions() -> Vec<String> {
    vec![
        "user:read".to_string(),
        "user:write".to_string(),
        "user:delete".to_string(),
        "group:read".to_string(),
        "group:write".to_string(),
        "network:read".to_string(),
        "network:write".to_string(),
        "system:read".to_string(),
        "system:write".to_string(),
        "portal:access".to_string(),
        "*".to_string(),
    ]
}

#[tauri::command]
pub async fn upload_id(store: State<'_, UserStore>, username: String, file_path: String) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        
        if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
            if user.login_history.is_empty() {
                 return Err("User must be tagged (logged in via portal) before uploading ID.".to_string());
            }
            user.id_document_path = Some(file_path);
        } else {
            return Err("User not found".to_string());
        }
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub async fn update_user_status(store: State<'_, UserStore>, username: String, active: bool, approved: bool) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
            user.is_active = active;
            user.is_approved = approved;
        } else {
            return Err("User not found".to_string());
        }
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub async fn update_user_groups(store: State<'_, UserStore>, username: String, groups: Vec<String>) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
            user.groups = groups;
        } else {
            return Err("User not found".to_string());
        }
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub fn get_user_settings(store: State<UserStore>) -> Result<UserSettings, String> {
    let db = store.db.lock().map_err(|e| e.to_string())?;
    Ok(db.settings.clone())
}

#[tauri::command]
pub async fn set_user_settings(store: State<'_, UserStore>, settings: UserSettings) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        db.settings = settings;
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub async fn add_user(store: State<'_, UserStore>, user: User) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        if db.users.iter().any(|u| u.username == user.username) {
            return Err("Username already exists".to_string());
        }
        db.users.push(user);
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_user(store: State<'_, UserStore>, username: String) -> Result<(), String>{
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        db.users.retain(|u| u.username != username);
    }
    store.persist().await?;
    // Explicit DB Delete
    let _: Option<User> = crate::sysmodules::db::get().delete(("users", &username)).await.ok().flatten();
    Ok(())
}

#[tauri::command]
pub async fn change_password(store: State<'_, UserStore>, username: String, new_password: String) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
            user.password_hash = new_password; // Hash in production!
        } else {
            return Err("User not found".to_string());
        }
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub async fn update_user_profile(store: State<'_, UserStore>, username: String, nickname: Option<String>, email: Option<String>) -> Result<(), String> {
    {
        let mut db = store.db.lock().map_err(|e| e.to_string())?;
        if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
            user.nickname = nickname;
            user.email = email;
        } else {
            return Err("User not found".to_string());
        }
    }
    store.persist().await?;
    Ok(())
}

#[tauri::command]
pub fn user_exists(store: State<UserStore>, username: String) -> Result<bool, String> {
    let db = store.db.lock().map_err(|e| e.to_string())?;
    Ok(db.users.iter().any(|u| u.username == username))
}


#[tauri::command]
pub fn sort_users_by(store: State<UserStore>, field: String, ascending: bool) -> Result<Vec<User>, String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    match field.as_str() {
        "username" => {
            if ascending {
                db.users.sort_by(|a, b| a.username.cmp(&b.username));
            } else {
                db.users.sort_by(|a, b| b.username.cmp(&a.username));
            }
        },
        "role" => {
            if ascending {
                db.users.sort_by(|a, b| a.role.cmp(&b.role));
            } else {
                db.users.sort_by(|a, b| b.role.cmp(&a.role));
            }
        },
        "group" => {
            if ascending {
                db.users.sort_by(|a, b| a.groups.join(",").cmp(&b.groups.join(",")));
            } else {
                db.users.sort_by(|a, b| b.groups.join(",").cmp(&a.groups.join(",")));
            }
        },
        _ => return Err("Invalid sort field".to_string()),
    }
    Ok(db.users.clone())
}