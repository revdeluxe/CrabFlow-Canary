use tauri::State;
use crate::user_management::user::{UserStore, User};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct SessionStore {
    pub sessions: Mutex<HashMap<String, User>>, // Token -> User
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub user: Option<User>,
}

#[tauri::command]
pub fn login(
    user_store: State<UserStore>, 
    session_store: State<SessionStore>, 
    req: LoginRequest
) -> Result<LoginResponse, String> {
    let db = user_store.db.lock().map_err(|e| e.to_string())?;
    
    if let Some(user) = db.users.iter().find(|u| u.username == req.username) {
        // In a real app, verify hash. Here we use simple string comparison as requested.
        if user.password_hash == req.password {
            if !user.is_active {
                return Ok(LoginResponse {
                    success: false,
                    message: "Account is disabled".to_string(),
                    token: None,
                    user: None,
                });
            }
            if !user.is_approved {
                return Ok(LoginResponse {
                    success: false,
                    message: "Account is pending approval".to_string(),
                    token: None,
                    user: None,
                });
            }

            // Generate Token
            let token = Uuid::new_v4().to_string();
            
            // Store Session
            let mut sessions = session_store.sessions.lock().map_err(|e| e.to_string())?;
            sessions.insert(token.clone(), user.clone());

            return Ok(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
                token: Some(token),
                user: Some(user.clone()),
            });
        }
    }

    Ok(LoginResponse {
        success: false,
        message: "Invalid credentials".to_string(),
        token: None,
        user: None,
    })
}

#[tauri::command]
pub fn logout(session_store: State<SessionStore>, token: String) -> Result<(), String> {
    let mut sessions = session_store.sessions.lock().map_err(|e| e.to_string())?;
    sessions.remove(&token);
    Ok(())
}

#[tauri::command]
pub fn check_auth(session_store: State<SessionStore>, token: String) -> Result<Option<User>, String> {
    let sessions = session_store.sessions.lock().map_err(|e| e.to_string())?;
    Ok(sessions.get(&token).cloned())
}

#[tauri::command]
pub fn register_user(store: State<UserStore>, username: String, password: String) -> Result<String, String> {
    let mut db = store.db.lock().map_err(|e| e.to_string())?;
    
    if db.users.iter().any(|u| u.username == username) {
        return Err("Username already exists".to_string());
    }

    let auto_approve = db.settings.auto_approve_new_users;

    let new_user = User {
        username: username.clone(),
        password_hash: password, // Hash this in production!
        role: "user".to_string(),
        groups: vec![],
        is_active: true,
        is_approved: auto_approve,
        login_history: vec![],
        id_document_path: None,
    };

    db.users.push(new_user);
    drop(db);
    store.save()?;

    if auto_approve {
        Ok("Registration successful".to_string())
    } else {
        Ok("Registration successful. Please wait for admin approval.".to_string())
    }
}
