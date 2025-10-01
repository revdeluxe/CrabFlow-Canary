#[macro_use] extern crate rocket;

use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct Context {
    title : String,
    message : String,
}

#[get("/")]
fn index() -> Template {
    let context = Context {
        title: "CrabFlow Canary".to_string(),
        message: "Welcome to the Rocket + Tera Powered UI".to_string(),
    };
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}
