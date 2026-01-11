use crate::sysmodules::config::load_logging_config;
use crate::sysmodules::paths;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{OnceLock, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use chrono::Local;
use serde::Serialize;

static LOG_SENDER: OnceLock<Sender<LogMessage>> = OnceLock::new();
static LOG_LEVEL: AtomicU8 = AtomicU8::new(1); // Default INFO (0=DEBUG, 1=INFO, 2=WARN, 3=ERROR)
static MEMORY_LOGS: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());

struct LogMessage {
    level: String,
    message: String,
    timestamp: String,
}

#[derive(Serialize, Clone)]
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
    // Ensure logs directory exists
    let logs_dir = paths::get_logs_dir();
    if let Err(e) = std::fs::create_dir_all(&logs_dir) {
        eprintln!("Failed to create logs directory: {}", e);
    }

    let (tx, rx) = channel::<LogMessage>();
    
    // Initialize Sender
    if LOG_SENDER.set(tx).is_err() {
        eprintln!("Failed to set global logger sender");
        return;
    }

    match load_logging_config() {
        Ok(cfg) => {
            println!("Log level: {}", cfg.level);
            println!("Log file: {}", cfg.file);
            LOG_LEVEL.store(level_str_to_u8(&cfg.level), Ordering::Relaxed);
            
            let file_path = paths::get_log_path(&cfg.file);
            
            // Spawn background logger thread
            thread::spawn(move || {
                // Keep file open in the thread context to avoid repeated open/close syscalls
                let mut file_handle = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_path)
                    .ok(); // Log error if fails?
                
                for msg in rx {
                    // 1. Write to File
                    if let Some(ref mut file) = file_handle {
                         let _ = writeln!(file, "{} [{}]: {}", msg.timestamp, msg.level, msg.message);
                    } else {
                        // Retry opening if it failed initially or was closed?
                        // For simplicity, try to reopen on every write if handle is missing, 
                        // but ideally we keep it open.
                         if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&file_path) {
                            let _ = writeln!(f, "{} [{}]: {}", msg.timestamp, msg.level, msg.message);
                            file_handle = Some(f);
                         }
                    }
                    
                    // 2. Write to Memory Buffer (Circular Buffer for UI)
                    let entry = LogEntry {
                        timestamp: msg.timestamp,
                        level: msg.level,
                        message: msg.message,
                    };
                    
                    if let Ok(mut logs) = MEMORY_LOGS.lock() {
                        if logs.len() >= 1000 {
                            logs.remove(0);
                        }
                        logs.push(entry);
                    }
                }
            });
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

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    if let Some(sender) = LOG_SENDER.get() {
        let _ = sender.send(LogMessage {
            level: level.to_string(),
            message: message.to_string(),
            timestamp,
        });
    } else {
        // Fallback or early init logging
        println!("[{}] {}", level, message);
    }
}

#[tauri::command]
pub fn get_logs(limit: usize) -> Vec<LogEntry> {
    if let Ok(logs) = MEMORY_LOGS.lock() {
        logs.iter().rev().take(limit).cloned().collect()
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn clear_logs() -> Result<(), String> {
    if let Ok(mut logs) = MEMORY_LOGS.lock() {
        logs.clear();
    }
    // Ideally clear the file too?
    Ok(())
}


pub fn log_info(message: &str) {
    if should_log("INFO") {
        println!("[INFO]: {}", message);
        save_log_to_file("INFO", message);
    }
}

pub fn log_warn(message: &str) {
    if should_log("WARN") {
        println!("[WARN]: {}", message);
        save_log_to_file("WARN", message);
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
pub fn get_logs_legacy(_limit: usize) -> Result<Vec<LogEntry>, String> {
    // Deprecated: Uses memory buffer now
    Ok(vec![])
}

