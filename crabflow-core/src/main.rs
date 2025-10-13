#[macro_use]
extern crate rocket;

mod settings;
use getifaddrs::getifaddrs;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use settings::Settings;

#[derive(Serialize)]
struct Context {
    title: String,
    message: String,
    interfaces: Vec<String>, // ← this must match the template
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
fn index() -> Template {
    let context = Context {
        title: "CrabFlow Canary".to_string(),
        message: "Welcome to the Rocket + Tera Powered UI".to_string(),
        interfaces: get_interfaces(),
    };
    Template::render("index", &context)
}

// ---- EXECUTORS ----
#[launch]
fn rocket() -> _ {
    let settings = Settings::load_or_create("CrabFlow.json");

    rocket::build()
        .attach(Template::fairing())
        .manage(settings)
        .mount("/", routes![index])
}
