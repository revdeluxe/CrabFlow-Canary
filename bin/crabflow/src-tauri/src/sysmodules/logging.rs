use crate::sysmodules::config::load_logging_config;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};
use dotenv::var;
use chrono::Local;
use serde::Serialize;

static LOG_FILE_PATH: OnceLock<String> = OnceLock::new();
static LOG_LEVEL: AtomicU8 = AtomicU8::new(1); // Default INFO (0=DEBUG, 1=INFO, 2=WARN, 3=ERROR)

#[derive(Serialize)]
pub struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn level_str_to_u8(level: &str) -> u8 {
    match level.to_uppercase().as_str() {
        "DEBUG" => 0,
        "INFO" => 1,
        "WARN" => 2,
        "ERROR" => 3,
        _ => 1,
    }
}

pub fn init_logging() {
    dotenv::dotenv().ok();

    let log_dir = var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());

    match load_logging_config() {
        Ok(cfg) => {
            println!("Log level: {}", cfg.level);
            println!("Log file: {}", cfg.file);

            let full_path = format!("{}/{}", log_dir, cfg.file);
            // Only set if not already set, or we need a way to update it.
            // OnceLock cannot be updated. For now assume file path doesn't change at runtime.
            LOG_FILE_PATH.get_or_init(|| full_path);
            
            LOG_LEVEL.store(level_str_to_u8(&cfg.level), Ordering::Relaxed);
        }
        Err(e) => eprintln!("Logging config error: {}", e),
    }
}

#[tauri::command]
pub fn reload_logging_config() {
    if let Ok(cfg) = load_logging_config() {
        LOG_LEVEL.store(level_str_to_u8(&cfg.level), Ordering::Relaxed);
        log_info(&format!("Log level updated to {}", cfg.level));
    }
}

fn should_log(level: &str) -> bool {
    let current = LOG_LEVEL.load(Ordering::Relaxed);
    let target = level_str_to_u8(level);
    target >= current
}

fn save_log_to_file(level: &str, message: &str) {
    if !should_log(level) && level != "EVENT" { return; }

    if let Some(path) = LOG_FILE_PATH.get() {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(file, "{} [{}]: {}", now, level, message);
        }
    }
}

pub fn log_info(message: &str) {
    if should_log("INFO") {
        println!("[INFO]: {}", message);
        save_log_to_file("INFO", message);
    }
}

pub fn log_error(message: &str) {
    if should_log("ERROR") {
        eprintln!("[ERROR]: {}", message);
        save_log_to_file("ERROR", message);
    }
}

pub fn log_debug(message: &str) {
    if should_log("DEBUG") {
        println!("[DEBUG]: {}", message);
        save_log_to_file("DEBUG", message);
    }
}

pub fn log_event(category: String, action: String, details: String) {
    let message = format!("[{}] {} - {}", category, action, details);
    println!("[EVENT]: {}", message);
    save_log_to_file("EVENT", &message);
}

#[tauri::command]
pub fn get_logs(limit: usize) -> Result<Vec<LogEntry>, String> {
    if let Some(path) = LOG_FILE_PATH.get() {
        if let Ok(content) = std::fs::read_to_string(path) {
            let lines: Vec<&str> = content.lines().collect();
            let start = if lines.len() > limit { lines.len() - limit } else { 0 };
            
            let mut logs = Vec::new();
            for line in &lines[start..] {
                // Parse line: "YYYY-MM-DD HH:MM:SS [LEVEL]: MESSAGE"
                let parts: Vec<&str> = line.splitn(3, ' ').collect();
                if parts.len() >= 3 {
                    let timestamp = format!("{} {}", parts[0], parts[1]);
                    let level_part = parts[2]; // "[LEVEL]: MESSAGE" or just "[LEVEL]:"
                    
                    let level_split: Vec<&str> = level_part.splitn(2, ": ").collect();
                    if level_split.len() == 2 {
                        let level = level_split[0].trim_matches(|c| c == '[' || c == ']');
                        let message = level_split[1];
                        logs.push(LogEntry {
                            timestamp,
                            level: level.to_string(),
                            message: message.to_string(),
                        });
                    } else {
                        // Fallback for malformed lines
                        logs.push(LogEntry {
                            timestamp,
                            level: "UNKNOWN".to_string(),
                            message: level_part.to_string(),
                        });
                    }
                } else {
                     // Fallback for very malformed lines
                     logs.push(LogEntry {
                        timestamp: "".to_string(),
                        level: "UNKNOWN".to_string(),
                        message: line.to_string(),
                    });
                }
            }
            // Reverse to show newest first
            logs.reverse();
            Ok(logs)
        } else {
            Ok(vec![])
        }
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub fn clear_logs() -> Result<(), String> {
    if let Some(path) = LOG_FILE_PATH.get() {
        // Overwrite with empty string
        std::fs::write(path, "").map_err(|e| e.to_string())?;
        log_info("Logs cleared by user");
        Ok(())
    } else {
        Err("Log file not initialized".to_string())
    }
}
