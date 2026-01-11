use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Emitter};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotificationPayload {
    pub title: String,
    pub message: String,
    pub level: String, // "info", "success", "warning", "error"
    pub timestamp: u64,
}

pub fn send_notification(app_handle: &AppHandle, title: &str, message: &str, level: &str) {
    // Log to system logs as well
    crate::sysmodules::logging::log_event("notify".into(), title.to_string(), format!("{} [{}]", message, level));

    let start = SystemTime::now();
    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let payload = NotificationPayload {
        title: title.to_string(),
        message: message.to_string(),
        level: level.to_string(),
        timestamp,
    };

    // Broadcast to all windows
    let _ = app_handle.emit("notification-event", payload);
}
