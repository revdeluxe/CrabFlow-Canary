use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
    http::{Method, HeaderValue},
};
use tower_http::cors::CorsLayer;
use std::sync::{Arc, Mutex};
use crate::user_management::user::{UserStore, User};
use crate::user_management::auth::{SessionStore, LoginRequest, LoginResponse};
use crate::sysmodules::logging;
use crate::network::{dhcp, dns, monitor};
use sysinfo::System;
use serde_json::{Value, json};
use uuid::Uuid;

// Shared State Container
#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStore,
    pub session_store: SessionStore,
    pub system: Arc<Mutex<System>>,
}

pub async fn start_server(user_store: UserStore, session_store: SessionStore) {
    let system = Arc::new(Mutex::new(System::new_all()));
    
    let state = AppState {
        user_store,
        session_store,
        system,
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:1420".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/logs", get(get_logs))
        .route("/api/system/status", get(get_system_status))
        .route("/api/dhcp/leases", get(list_leases))
        .route("/api/dns/records", get(list_records))
        // Auth Routes
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/check", post(check_auth_handler))
        .route("/api/users", get(list_users_handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();
    logging::log_info("HTTP API Server listening on http://127.0.0.1:3030");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok", "mode": "admin" }))
}

async fn get_logs() -> Json<Value> {
    let logs = crate::sysmodules::logging::get_logs(100);
    Json(json!(logs))
}

async fn get_system_status(State(state): State<AppState>) -> Json<Value> {
    let mut sys = state.system.lock().unwrap();
    let status = monitor::get_system_status_impl(&mut sys);
    Json(json!(status))
}

async fn list_leases() -> Json<Value> {
    let leases = dhcp::list_leases();
    Json(json!(leases))
}

async fn list_records() -> Json<Value> {
    let records = dns::list_records();
    Json(json!(records))
}

// Auth Handlers

#[derive(serde::Deserialize)]
struct TokenRequest {
    token: String,
}

async fn check_auth_handler(
    State(state): State<AppState>,
    Json(req): Json<TokenRequest>,
) -> Json<Option<User>> {
    if let Ok(sessions) = state.session_store.sessions.lock() {
        Json(sessions.get(&req.token).cloned())
    } else {
        Json(None)
    }
}

async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let db = match state.user_store.db.lock() {
        Ok(db) => db,
        Err(_) => return Json(LoginResponse {
            success: false,
            message: "Database lock error".to_string(),
            token: None,
            user: None,
        }),
    };

    if let Some(user) = db.users.iter().find(|u| u.username == req.username) {
        if user.password_hash == req.password {
             if !user.is_active {
                return Json(LoginResponse {
                    success: false,
                    message: "Account is disabled".to_string(),
                    token: None,
                    user: None,
                });
            }
            if !user.is_approved {
                return Json(LoginResponse {
                    success: false,
                    message: "Account is pending approval".to_string(),
                    token: None,
                    user: None,
                });
            }

            let token = Uuid::new_v4().to_string();
            if let Ok(mut sessions) = state.session_store.sessions.lock() {
                sessions.insert(token.clone(), user.clone());
            }

            return Json(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
                token: Some(token),
                user: Some(user.clone()),
            });
        }
    }

    Json(LoginResponse {
        success: false,
        message: "Invalid credentials".to_string(),
        token: None,
        user: None,
    })
}

async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>, // Reusing LoginRequest since it has username/password
) -> Json<Value> {
    let username = req.username;
    let password = req.password;
    
    let auto_approve;
    {
        let mut db = match state.user_store.db.lock() {
            Ok(db) => db,
            Err(e) => return Json(json!({"error": e.to_string()})),
        };
        
        if db.users.iter().any(|u| u.username == username) {
            return Json(json!({"error": "Username already exists"}));
        }

        auto_approve = db.settings.auto_approve_new_users;

        let new_user = User {
            username: username.clone(),
            nickname: None,
            email: None,
            password_hash: password,
            role: "user".to_string(),
            groups: vec![],
            is_active: true,
            is_approved: auto_approve,
            login_history: vec![],
            id_document_path: None,
        };

        db.users.push(new_user);
    } // drop lock

    let _ = state.user_store.persist().await;

    if auto_approve {
        Json(json!({"message": "Registration successful"}))
    } else {
        Json(json!({"message": "Registration successful. Please wait for admin approval."}))
    }
}

async fn list_users_handler(State(state): State<AppState>) -> Json<Vec<User>> {
    let db = state.user_store.db.lock().unwrap();
    Json(db.users.clone())
}
