// src/routes/index.rs
// Handler for the home/index page.

// This route serves the content for the "/" path
#[rocket::get("/")]
pub fn index() -> &'static str {
    "Hello, Welcome to CrabFlow Core! This is the public home page."
}