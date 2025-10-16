#[macro_use]
extern crate rocket;
mod monitor;

mod settings;

use monitor::gather_dashboard_context;
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

    let _ = std::fs::write(
        "CrabFlow.json",
        serde_json::to_string_pretty(&settings).unwrap(),
    );

    Json(settings)
}

// ---- FUNCTIONS ----

#[get("/")]
fn landing_page() -> Template {
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
    println!("The default theme is: {}", theme);

    let context = gather_dashboard_context();
    Template::render("dashboard", &context)
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

// ---- EXECUTORS ----
#[launch]
fn rocket() -> _ {
    let settings = Settings::load_or_create("CrabFlow.json");

    rocket::build()
        .attach(Template::fairing())
        .manage(settings)
        .mount(
            "/",
            routes![
                landing_page,
                dashboard_page,
                settings_page,
                get_settings,
                save_settings
            ],
        )
        // The path needs to be relative to this file's location (src/main.rs)
        .mount("/static", FileServer::from(relative!("/templates/static")))
}
