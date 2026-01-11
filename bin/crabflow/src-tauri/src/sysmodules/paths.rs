use std::path::PathBuf;
use std::fs;
use std::env;

/// Get the installation directory (where the executable is located)
/// This is the base path for all data directories
pub fn get_install_dir() -> PathBuf {
    // Get the path of the current executable
    if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            return parent.to_path_buf();
        }
    }
    // Fallback to current directory
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Get the data directory (same as install dir for portable deployment)
/// Structure:
/// ./config/
/// ./db/
/// ./logs/
/// ./crabflow.exe
pub fn get_data_dir() -> PathBuf {
    get_install_dir()
}

/// Get the config directory
pub fn get_config_dir() -> PathBuf {
    get_install_dir().join("config")
}

/// Get the database directory
pub fn get_db_dir() -> PathBuf {
    get_install_dir().join("db")
}

/// Get the logs directory
pub fn get_logs_dir() -> PathBuf {
    get_install_dir().join("logs")
}

/// Get the full path to a config file
pub fn get_config_path(filename: &str) -> PathBuf {
    get_config_dir().join(filename)
}

/// Get the full path to a database file
pub fn get_db_path(filename: &str) -> PathBuf {
    get_db_dir().join(filename)
}

/// Get the full path to a log file
pub fn get_log_path(filename: &str) -> PathBuf {
    get_logs_dir().join(filename)
}

/// Initialize all data directories
pub fn init_data_dir() -> std::io::Result<()> {
    let config_dir = get_config_dir();
    let db_dir = get_db_dir();
    let logs_dir = get_logs_dir();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir)?;
    }
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir)?;
    }
    Ok(())
}
