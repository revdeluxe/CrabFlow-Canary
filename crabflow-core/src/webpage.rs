// src/webpage.rs
use rocket::Route;

// IMPORTANT: We use 'crate::routes' because 'routes' is defined in main.rs
use crate::routes; 

pub fn routes() -> Vec<Route> {
    rocket::routes![
        routes::index::index,
        routes::about::about
        // Add other pages here
    ]
}