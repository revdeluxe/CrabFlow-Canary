#[macro_use]
extern crate rocket;
mod monitor;

mod settings;

use monitor::gather_dashboard_context;
use rocket::State;
use rocket::fs::{FileServer, relative};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use settings::Settings;

#[derive(Serialize)]
struct Context {
    title: String,
    message: String,
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
        .mount("/", routes![landing_page, dashboard_page, settings_page])
        // The path needs to be relative to this file's location (src/main.rs)
        .mount("/static", FileServer::from(relative!("/templates/static")))
}
