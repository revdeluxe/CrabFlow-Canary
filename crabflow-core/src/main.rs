#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;
mod monitor;

mod settings;

use monitor::{gather_dashboard_context, get_network_traffic};
use rocket::State;
use rocket::fs::{FileServer, relative};
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use settings::Settings;

#[derive(Serialize)]
struct Context {
    title: String,
    message: String,
}

#[derive(Serialize)]
struct DhcpLease {
    ip_address: String,
    mac_address: String,
    hostname: String,
    lease_time: u64,
}

// ---- API ----
#[get("/api/settings")]
fn get_settings(settings: &State<Settings>) -> Json<Settings> {
    Json(settings.inner().clone())
}

#[post("/api/settings", data = "<new_settings>")]
fn save_settings(new_settings: Json<Settings>, settings: &State<Settings>) -> Json<Settings> {
    let mut settings = settings.inner().clone();
    settings.app_name = new_settings.app_name.clone();
    settings.port = new_settings.port;
    settings.enable_ui = new_settings.enable_ui;
    settings.sdnc_mode = new_settings.sdnc_mode.clone();
    settings.log_level = new_settings.log_level.clone();
    settings.default_theme = new_settings.default_theme.clone();

    let _ = settings.save("settings.json");

    Json(settings)
}

#[get("/api/traffic")]
fn traffic_data() -> Json<Vec<monitor::InterfaceTraffic>> {
    Json(get_network_traffic())
}

#[get("/api/dhcp/leases")]
fn get_dhcp_leases() -> Json<Vec<DhcpLease>> {
    let leases = vec![
        DhcpLease {
            ip_address: "192.168.1.10".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            hostname: "device-1".to_string(),
            lease_time: 3600,
        },
        DhcpLease {
            ip_address: "192.168.1.11".to_string(),
            mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
            hostname: "device-2".to_string(),
            lease_time: 7200,
        },
    ];
    Json(leases)
}

// ---- FUNCTIONS ----

#[get("/")]
fn landing_page() -> Template {
    info!("Serving landing page");
    let context = Context {
        title: "CrabFlow Canary".to_string(),
        message: "Welcome to the Rocket + Tera Powered UI".to_string(),
    };
    Template::render("index", &context)
}

#[get("/dashboard")]
fn dashboard_page(settings: &State<Settings>) -> Template {
    // You are correctly accessing the settings here!
    let theme = &settings.default_theme;
    info!("The default theme is: {}", theme);

    let context = gather_dashboard_context();
    Template::render("dashboard", &context)
}

#[get("/device-manager")]
fn device_manager_page() -> Template {
    let context = Context {
        title: "Device Manager".to_string(),
        message: "".to_string(),
    };
    Template::render("device-manager", &context)
}

#[get("/dhcp-server")]
fn dhcp_server_page() -> Template {
    let context = Context {
        title: "DHCP Server".to_string(),
        message: "".to_string(),
    };
    Template::render("dhcp-server", &context)
}

#[derive(Serialize)]
struct SettingsContext {
    title: String,
    settings: settings::Settings,
}

// This new route will render your settings page.
// It passes the settings object to the template so you can display them.
#[get("/settings")]
fn settings_page(settings: &State<Settings>) -> Template {
    let context = SettingsContext {
        title: "Settings".to_string(),
        settings: settings.inner().clone(),
    };
    Template::render("settings", &context)
}

// ---- CATCHERS ----

#[catch(404)]
fn not_found() -> Template {
    warn!("404 Not Found");
    let context = Context {
        title: "404 Not Found".to_string(),
        message: "The page you are looking for could not be found.".to_string(),
    };
    Template::render("404", &context)
}

#[catch(403)]
fn forbidden() -> Template {
    warn!("403 Forbidden");
    let context = Context {
        title: "403 Forbidden".to_string(),
        message: "You do not have permission to access this page.".to_string(),
    };
    Template::render("403", &context)
}

#[catch(500)]
fn internal_error() -> Template {
    error!("500 Internal Server Error");
    let context = Context {
        title: "500 Internal Server Error".to_string(),
        message: "An internal server error occurred.".to_string(),
    };
    Template::render("500", &context)
}

#[catch(502)]
fn bad_gateway() -> Template {
    error!("502 Bad Gateway");
    let context = Context {
        title: "502 Bad Gateway".to_string(),
        message: "The server received an invalid response from an upstream server.".to_string(),
    };
    Template::render("502", &context)
}

// ---- EXECUTORS ----
use env_logger::Builder;
use log::LevelFilter;

// ...

fn get_log_level(log_level: &str) -> LevelFilter {
    match log_level.to_lowercase().as_str() {
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "debug" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    }
}

// ...

#[launch]
fn rocket() -> _ {
    let settings = Settings::load_or_create("CrabFlow.json");

    // Initialize logger
    let log_level = get_log_level(&settings.log_level);
    let mut builder = Builder::new();
    builder.filter_level(log_level);
    builder.init();

    rocket::build()
        .attach(Template::fairing())
        .manage(settings)
        .mount(
            "/",
            routes![
                landing_page,
                dashboard_page,
                settings_page,
                device_manager_page,
                dhcp_server_page,
                get_settings,
                save_settings,
                traffic_data,
                get_dhcp_leases
            ],
        )
        // The path needs to be relative to this file's location (src/main.rs)
        .mount("/static", FileServer::from(relative!("/templates/static")))
        .register(
            "/",
            catchers![not_found, forbidden, internal_error, bad_gateway],
        )
}
