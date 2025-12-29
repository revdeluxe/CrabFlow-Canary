// src-tauri/src/render.rs

use serde::Serialize;
use tauri::Window;

/// Generic render payload
#[derive(Serialize)]
pub struct RenderPayload<T> {
    pub kind: String,
    pub data: T,
}

/// Emit data to the frontend
pub fn emit<T: Serialize>(window: &Window, kind: &str, data: T) {
    let payload = RenderPayload {
        kind: kind.to_string(),
        data,
    };
    if let Err(e) = window.emit("crabflow://render", payload) {
        eprintln!("Render emit failed: {}", e);
    }
}
