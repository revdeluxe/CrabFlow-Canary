use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
    http::{Method, HeaderValue},
};
use tower_http::cors::CorsLayer;
use std::sync::{Arc, Mutex};
use crate::user_management::user::UserStore;
use crate::sysmodules::logging;
use crate::network::{dhcp, dns, monitor};
use sysinfo::System;
use serde_json::{Value, json};

// Shared State Container
#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStore,
    pub system: Arc<Mutex<System>>,
}

pub async fn start_server(user_store: UserStore) {
    let system = Arc::new(Mutex::new(System::new_all()));
    
    let state = AppState {
        user_store,
        system,
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:1420".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/logs", get(get_logs))
        .route("/api/system/status", get(get_system_status))
        .route("/api/dhcp/leases", get(list_leases))
        .route("/api/dns/records", get(list_records))
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
