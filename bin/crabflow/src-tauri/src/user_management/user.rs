use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::State;

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
    pub role: String, // "admin", "user", "guest"
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
    pub db_path: PathBuf,
    pub db: Arc<Mutex<UserDatabase>>,
}

impl UserStore {
    pub fn new(db_path: PathBuf) -> Self {
        let db = if db_path.exists() {
            let content = fs::read_to_string(&db_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| UserDatabase::default())
        } else {
            UserDatabase::default()
        };
        
        Self {
            db_path,
            db: Arc::new(Mutex::new(db)),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let db = self.db.lock().map_err(|e| e.to_string())?;
        let content = serde_json::to_string_pretty(&*db).map_err(|e| e.to_string())?;
        if let Some(parent) = self.db_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.db_path, content).map_err(|e| e.to_string())?;
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
                    role: "admin".to_string(),
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
pub fn add_group(store: State<UserStore>, name: String, description: String, permissions: Vec<String>) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    if db.groups.iter().any(|g| g.name == name) {
        return Err("Group already exists".to_string());
    }
    db.groups.push(Group { name, description, permissions });
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn update_group(store: State<UserStore>, name: String, description: String, permissions: Vec<String>) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    if let Some(group) = db.groups.iter_mut().find(|g| g.name == name) {
        group.description = description;
        group.permissions = permissions;
    } else {
        return Err("Group not found".to_string());
    }
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn delete_group(store: State<UserStore>, name: String) -> Result<(), String> {
    if name == "admin" {
        return Err("Cannot delete admin group".to_string());
    }
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    db.groups.retain(|g| g.name != name);
    drop(db);
    store.save()?;
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
pub fn upload_id(store: State<UserStore>, username: String, file_path: String) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    
    if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
        if user.login_history.is_empty() {
             return Err("User must be tagged (logged in via portal) before uploading ID.".to_string());
        }
        user.id_document_path = Some(file_path);
    } else {
        return Err("User not found".to_string());
    }
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn update_user_status(store: State<UserStore>, username: String, active: bool, approved: bool) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
        user.is_active = active;
        user.is_approved = approved;
    } else {
        return Err("User not found".to_string());
    }
    drop(db); // Unlock before save
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn update_user_groups(store: State<UserStore>, username: String, groups: Vec<String>) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
        user.groups = groups;
    } else {
        return Err("User not found".to_string());
    }
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn get_user_settings(store: State<UserStore>) -> Result<UserSettings, String> {
    let db = store.db.lock().map_err(|e| e.to_string())?;
    Ok(db.settings.clone())
}

#[tauri::command]
pub fn set_user_settings(store: State<UserStore>, settings: UserSettings) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    db.settings = settings;
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn add_user(store: State<UserStore>, user: User) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    if db.users.iter().any(|u| u.username == user.username) {
        return Err("Username already exists".to_string());
    }
    db.users.push(user);
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn remove_user(store: State<UserStore>, username: String) -> Result<(), String>{
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    db.users.retain(|u| u.username != username);
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn change_password(store: State<UserStore>, username: String, new_password: String) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
        user.password_hash = new_password; // Hash in production!
    } else {
        return Err("User not found".to_string());
    }
    drop(db);
    store.save()?;
    Ok(())
}

#[tauri::command]
pub fn update_user_profile(store: State<UserStore>, username: String, nickname: Option<String>, email: Option<String>) -> Result<(), String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    if let Some(user) = db.users.iter_mut().find(|u| u.username == username) {
        user.nickname = nickname;
        user.email = email;
    } else {
        return Err("User not found".to_string());
    }
    drop(db);
    store.save()?;
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