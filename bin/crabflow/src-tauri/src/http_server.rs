use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
    http::{Method, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
};
use tower_http::cors::CorsLayer;
use std::sync::{Arc, Mutex};
use crate::user_management::user::{UserStore, User};
use crate::user_management::auth::{SessionStore, LoginRequest, LoginResponse};
use crate::sysmodules::logging;
use crate::network::{dhcp, dns, monitor, acl};
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

    // Allow CORS from multiple origins for both Tauri app and captive portal clients
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
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
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/users", get(list_users_handler))
        // Portal API Routes (for browser access)
        .route("/api/portal/template", get(get_portal_template_handler))
        .route("/api/portal/tag", post(tag_user_handler))
        // Captive Portal Detection Routes (must match what devices check)
        .route("/connecttest.txt", get(captive_portal_windows))
        .route("/ncsi.txt", get(captive_portal_windows))
        .route("/hotspot-detect.html", get(captive_portal_apple))
        .route("/library/test/success.html", get(captive_portal_apple))
        .route("/generate_204", get(captive_portal_android))
        .route("/gen_204", get(captive_portal_android))
        // Captive Portal pages
        .route("/captive", get(captive_portal_page))
        .route("/login", get(captive_portal_login_page))
        .route("/portal/login", post(portal_login_handler))
        .layer(cors)
        .with_state(state);

    // Listen on all interfaces so captive portal clients can reach us
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    logging::log_info("HTTP API Server listening on http://0.0.0.0:3030");
    
    // Use into_make_service_with_connect_info to get client IP
    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await.unwrap();
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
    logging::log_info(&format!("Login attempt for user: {}", req.username));

    let db = match state.user_store.db.lock() {
        Ok(db) => db,
        Err(_) => {
            logging::log_error("Failed to lock user database");
            return Json(LoginResponse {
                success: false,
                message: "Database lock error".to_string(),
                token: None,
                user: None,
            });
        }
    };

    if let Some(user) = db.users.iter().find(|u| u.username == req.username) {
        if user.password_hash == req.password {
             if !user.is_active {
                logging::log_warn(&format!("Login failed: Account disabled for {}", req.username));
                return Json(LoginResponse {
                    success: false,
                    message: "Account is disabled".to_string(),
                    token: None,
                    user: None,
                });
            }
            if !user.is_approved {
                logging::log_warn(&format!("Login failed: Account pending approval for {}", req.username));
                return Json(LoginResponse {
                    success: false,
                    message: "Account is pending approval".to_string(),
                    token: None,
                    user: None,
                });
            }

            let token = Uuid::new_v4().to_string();
            match state.session_store.sessions.lock() {
                Ok(mut sessions) => {
                    sessions.insert(token.clone(), user.clone());
                    logging::log_info(&format!("Login successful for {}", req.username));
                }
                Err(e) => {
                    logging::log_error(&format!("Failed to lock session store: {}", e));
                    // Even if session store fails, we shouldn't crash, but login fails effectively
                    return Json(LoginResponse {
                        success: false,
                        message: "Session store error".to_string(),
                        token: None,
                        user: None,
                    });
                }
            }

            return Json(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
                token: Some(token),
                user: Some(user.clone()),
            });
        } else {
            logging::log_warn(&format!("Login failed: Invalid password for {}", req.username));
        }
    } else {
        logging::log_warn(&format!("Login failed: User not found {}", req.username));
    }

    Json(LoginResponse {
        success: false,
        message: "Invalid credentials".to_string(),
        token: None,
        user: None,
    })
}

use crate::user_management::permission::Role;

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
            role: Role::Guest,
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
    match state.user_store.db.lock() {
        Ok(db) => Json(db.users.clone()),
        Err(_) => Json(vec![]) // Return empty list on error instead of panic
    }
}

async fn logout_handler(
    State(state): State<AppState>,
    Json(req): Json<TokenRequest>,
) -> Json<Value> {
    match state.session_store.sessions.lock() {
        Ok(mut sessions) => {
            if sessions.remove(&req.token).is_some() {
                logging::log_info("Session logged out successfully");
                Json(json!({"success": true, "message": "Logged out successfully"}))
            } else {
                Json(json!({"success": true, "message": "Session not found but ok"}))
            }
        }
        Err(_) => Json(json!({"success": false, "message": "Session store error"}))
    }
}

async fn get_portal_template_handler() -> Json<Value> {
    match crate::network::cportal::get_portal_template() {
        Ok(html) => Json(json!(html)),
        Err(e) => Json(json!({"error": e}))
    }
}

#[derive(serde::Deserialize)]
struct TagUserRequest {
    username: String,
    ip: String,
    device_name: Option<String>,
}

async fn tag_user_handler(
    State(state): State<AppState>,
    Json(req): Json<TagUserRequest>,
) -> Json<Value> {
    // Simplified tag_user for HTTP - without Tauri AppHandle for notifications
    let mac = if req.ip == "127.0.0.1" || req.ip == "localhost" {
        "00:00:00:00:00:00".to_string()
    } else {
        match dhcp::get_mac_from_ip(&req.ip) {
            Some(m) => m,
            None => {
                logging::log_warn(&format!("Tagging user {} with unknown MAC (IP: {})", req.username, req.ip));
                "UNKNOWN_MAC".to_string()
            }
        }
    };

    {
        let mut db = match state.user_store.db.lock() {
            Ok(db) => db,
            Err(e) => return Json(json!({"success": false, "error": e.to_string()})),
        };

        if let Some(user) = db.users.iter_mut().find(|u| u.username == req.username) {
            use chrono::Utc;
            use crate::user_management::user::LoginRecord;
            
            let now = Utc::now();
            let record = LoginRecord {
                ip: req.ip.clone(),
                mac: mac.clone(),
                timestamp: now.to_rfc3339(),
                device_name: req.device_name,
            };
            user.login_history.push(record);
        } else {
            return Json(json!({"success": false, "error": format!("User not found: {}", req.username)}));
        }
    }

    // Whitelist the user so DNS stops hijacking them
    dns::authorize_ip(req.ip.clone());
    
    // Persist changes
    let _ = state.user_store.persist().await;

    logging::log_info(&format!("Tagged user {} from IP {}", req.username, req.ip));
    Json(json!({"success": true}))
}

// ============================================================================
// Captive Portal Detection Handlers
// ============================================================================

/// Windows Captive Portal Detection
/// Windows checks: http://www.msftconnecttest.com/connecttest.txt
/// Expected response when internet is available: "Microsoft Connect Test"
/// To trigger captive portal: Return anything else or redirect
async fn captive_portal_windows() -> Response {
    if acl::is_captive_portal_enabled() {
        // Return a redirect to trigger the "Sign in to network" prompt
        logging::log_debug("Windows captive portal detection - redirecting to portal");
        (
            StatusCode::FOUND,
            [("Location", "/captive")],
            "Redirecting to captive portal"
        ).into_response()
    } else {
        // Return the expected response to indicate internet is available
        (StatusCode::OK, "Microsoft Connect Test").into_response()
    }
}

/// Apple Captive Portal Detection
/// Apple devices check: http://captive.apple.com/hotspot-detect.html
/// Expected response when internet is available: "<HTML><HEAD><TITLE>Success</TITLE></HEAD><BODY>Success</BODY></HTML>"
/// To trigger captive portal: Return anything else
async fn captive_portal_apple() -> Response {
    if acl::is_captive_portal_enabled() {
        // Return the captive portal page content to trigger the prompt
        logging::log_debug("Apple captive portal detection - showing portal");
        Html(get_captive_portal_html()).into_response()
    } else {
        // Return the expected response
        Html("<HTML><HEAD><TITLE>Success</TITLE></HEAD><BODY>Success</BODY></HTML>".to_string()).into_response()
    }
}

/// Android Captive Portal Detection  
/// Android checks: http://connectivitycheck.gstatic.com/generate_204
/// Expected response when internet is available: HTTP 204 No Content
/// To trigger captive portal: Return HTTP 302 redirect or any other status
async fn captive_portal_android() -> Response {
    if acl::is_captive_portal_enabled() {
        // Return a redirect to trigger the captive portal
        logging::log_debug("Android captive portal detection - redirecting to portal");
        (
            StatusCode::FOUND,
            [("Location", "/captive")],
            ""
        ).into_response()
    } else {
        // Return 204 No Content to indicate internet is available
        StatusCode::NO_CONTENT.into_response()
    }
}

/// The actual captive portal page that users see
async fn captive_portal_page() -> Html<String> {
    Html(get_captive_portal_html())
}

fn get_captive_portal_html() -> String {
    // Try to load custom portal template
    if let Ok(template) = crate::network::cportal::get_portal_template() {
        return format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sign in to Network - CrabFlow</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }}
    </style>
</head>
<body>
    {}
    <script>
        async function handleLogin(event) {{
            event.preventDefault();
            const form = event.target;
            const username = form.username.value;
            const password = form.password.value;
            const errorDiv = document.getElementById('error-message');
            
            try {{
                const response = await fetch('/api/auth/login', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{ username, password }})
                }});
                const data = await response.json();
                
                if (data.success) {{
                    // Notify the network that this IP is authenticated
                    // The backend will handle the IP whitelisting
                    errorDiv.style.display = 'none';
                    alert('Login successful! You now have internet access.');
                    window.location.href = 'http://www.msftconnecttest.com/redirect';
                }} else {{
                    errorDiv.textContent = data.message || 'Login failed';
                    errorDiv.style.display = 'block';
                }}
            }} catch (e) {{
                errorDiv.textContent = 'Connection error. Please try again.';
                errorDiv.style.display = 'block';
            }}
        }}
    </script>
</body>
</html>
"#, template);
    }

    // Default captive portal page
    r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sign in to Network - CrabFlow</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            background: white;
            padding: 2rem;
            border-radius: 12px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            width: 100%;
            max-width: 400px;
            margin: 1rem;
        }
        h1 {
            color: #333;
            margin-bottom: 0.5rem;
            font-size: 1.5rem;
        }
        p {
            color: #666;
            margin-bottom: 1.5rem;
        }
        .form-group {
            margin-bottom: 1rem;
        }
        label {
            display: block;
            color: #333;
            margin-bottom: 0.25rem;
            font-weight: 500;
        }
        input {
            width: 100%;
            padding: 0.75rem;
            border: 1px solid #ddd;
            border-radius: 6px;
            font-size: 1rem;
        }
        input:focus {
            outline: none;
            border-color: #667eea;
        }
        button {
            width: 100%;
            padding: 0.75rem;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 6px;
            font-size: 1rem;
            cursor: pointer;
            font-weight: 500;
        }
        button:hover {
            opacity: 0.9;
        }
        .error {
            color: #dc3545;
            margin-top: 1rem;
            padding: 0.5rem;
            background: #f8d7da;
            border-radius: 4px;
            display: none;
        }
        .logo {
            text-align: center;
            margin-bottom: 1rem;
            font-size: 2rem;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="logo">🦀</div>
        <h1>Welcome to CrabFlow Network</h1>
        <p>Please sign in to access the internet.</p>
        
        <form id="login-form" onsubmit="handleLogin(event)">
            <div class="form-group">
                <label for="username">Username / Voucher Code</label>
                <input type="text" id="username" name="username" placeholder="Enter your username" required>
            </div>
            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" placeholder="Enter your password" required>
            </div>
            <button type="submit">Connect to Internet</button>
        </form>
        
        <div id="error-message" class="error"></div>
    </div>
    
    <script>
        async function handleLogin(event) {
            event.preventDefault();
            const form = event.target;
            const username = form.username.value;
            const password = form.password.value;
            const errorDiv = document.getElementById('error-message');
            
            try {
                const response = await fetch('/portal/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ username, password })
                });
                const data = await response.json();
                
                if (data.success) {
                    errorDiv.style.display = 'none';
                    alert('Login successful! You now have internet access.');
                    // Redirect to trigger connectivity recheck
                    window.location.href = 'http://www.msftconnecttest.com/redirect';
                } else {
                    errorDiv.textContent = data.message || 'Login failed';
                    errorDiv.style.display = 'block';
                }
            } catch (e) {
                errorDiv.textContent = 'Connection error. Please try again.';
                errorDiv.style.display = 'block';
            }
        }
    </script>
</body>
</html>
    "#.to_string()
}

/// Captive portal login page (alias for /captive)
async fn captive_portal_login_page() -> Html<String> {
    Html(get_captive_portal_html())
}

/// Portal login handler - authenticates user and authorizes their IP
async fn portal_login_handler(
    State(state): State<AppState>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    Json(req): Json<LoginRequest>,
) -> Json<Value> {
    let client_ip = addr.ip().to_string();
    logging::log_info(&format!("Portal login attempt from {} for user: {}", client_ip, req.username));

    let db = match state.user_store.db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Json(json!({
                "success": false,
                "message": "Database error"
            }));
        }
    };

    if let Some(user) = db.users.iter().find(|u| u.username == req.username) {
        if user.password_hash == req.password {
            if !user.is_active {
                return Json(json!({
                    "success": false,
                    "message": "Account is disabled"
                }));
            }
            if !user.is_approved {
                return Json(json!({
                    "success": false,
                    "message": "Account is pending approval"
                }));
            }

            // Authorize this IP for internet access
            dns::authorize_ip(client_ip.clone());
            
            logging::log_info(&format!("Portal login successful for {} from IP {}", req.username, client_ip));
            
            return Json(json!({
                "success": true,
                "message": "Login successful! You now have internet access."
            }));
        }
    }

    Json(json!({
        "success": false,
        "message": "Invalid credentials"
    }))
}
