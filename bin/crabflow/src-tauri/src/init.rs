// src-tauri/src/init.rs

use crate::network::acl;

/// Verify that the application is properly initialized
/// This function checks that data directories exist and are writable
pub fn does_system_files() -> bool {
    use crate::sysmodules::paths;
    
    // Initialize data directories
    if paths::init_data_dir().is_err() {
        return false;
    }
    
    // Initialize ACL configuration
    acl::init_acl();

    // Verify directories exist
    let config_dir = paths::get_config_dir();
    let db_dir = paths::get_db_dir();
    let logs_dir = paths::get_logs_dir();
    
    let kv_db_path = paths::get_db_path("crabflow.kv");
    if kv_db_path.exists() {
        crate::sysmodules::logging::log_debug(&format!("Contents of crabflow.kv directory: {:#?}", std::fs::read_dir(kv_db_path).map(|d| d.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).collect::<Vec<String>>())));
    }

    config_dir.exists() && db_dir.exists() && logs_dir.exists()
}
