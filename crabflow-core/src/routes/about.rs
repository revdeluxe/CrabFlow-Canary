// src/routes/about.rs
// Handler for the /about page.

// This route serves the content for the "/about" path
#[rocket::get("/about")]
pub fn about() -> &'static str {
    "CrabFlow is a powerful, modern application built with Rust and Tauri/Rocket."
}