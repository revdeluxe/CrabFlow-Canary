#[macro_use]
extern crate rocket;
mod monitor;

mod settings;
use getifaddrs::getifaddrs;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use settings::Settings;
use monitor::gather_dashboard_context;

#[derive(Serialize)]
struct Context {
    title: String,
    message: String
}

// ---- FUNCTIONS ----
#[tauri::command]
fn get_settings() -> Settings {
    Settings::load_or_create("CrabFlow.json")
}

fn get_interfaces() -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(ifaces) = getifaddrs() {
        for iface in ifaces {
            let ip = format!("{:?}", iface.address);
            names.push(format!("{} → {}", iface.name, ip));
        }
    }
    names
}

#[get("/")]
fn landing_page() -> Template {
    let context = Context {
        title: "CrabFlow Canary".to_string(),
        message: "Welcome to the Rocket + Tera Powered UI".to_string()
    };
    Template::render("index", &context)
}

#[get("/dashboard")]
fn dashboard_page() -> Template {
    let context = gather_dashboard_context();
    Template::render("dashboard", &context)
}

// ---- EXECUTORS ----
#[launch]
fn rocket() -> _ {
    let settings = Settings::load_or_create("CrabFlow.json");

    rocket::build()
        .attach(Template::fairing())
        .manage(settings)
        .mount("/", routes![landing_page, dashboard_page])
}
