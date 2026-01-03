// use std::path::Path;
use std::fs;
use crate::sysmodules::config::get_project_root;

pub fn initialize_user_management() {
    // Ensure the db directory exists
    let root = get_project_root();
    let db_path = root.join("db");
    if !db_path.exists() {
        let _ = fs::create_dir_all(db_path);
    }
    println!("User management initialized");
}
