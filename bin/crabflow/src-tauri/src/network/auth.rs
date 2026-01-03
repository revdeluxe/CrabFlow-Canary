// src-tauri/src/network/auth.rs
use serde::{Deserialize, Serialize};
// use crate::setup::wizard::load_setup;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Session {
    pub id: String,
    pub user: String,
    pub role: String, // "guest", "staff", "admin"
}

/*
#[tauri::command]
pub fn login(req: LoginRequest) -> Result<Session, String> {
    // Try to load config from setup
    if let Ok(config) = load_setup() {
        if req.username == config.admin_user && req.password == config.admin_pass {
             return Ok(Session {
                id: "sess-admin".into(),
                user: req.username,
                role: "admin".into(),
            });
        }
    }

    // Fallback / Hardcoded for dev (optional, maybe remove if strict)
    if req.username == "admin" && req.password == "secret" {
        Ok(Session {
            id: "sess-123".into(),
            user: req.username,
            role: "admin".into(),
        })
    } else if req.username == "user" && req.password == "password" {
        Ok(Session {
            id: "sess-456".into(),
            user: req.username,
            role: "guest".into(),
        })
    } else {
        Err("Invalid credentials".into())
    }
}
*/
