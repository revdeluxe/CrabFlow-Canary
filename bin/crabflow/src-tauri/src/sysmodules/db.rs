use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;
use std::sync::OnceLock;
use crate::sysmodules::paths;

pub static DB: OnceLock<Surreal<surrealdb::engine::local::Db>> = OnceLock::new();

pub async fn init() -> Result<(), String> {
    let db_path = paths::get_db_path("crabflow.kv");
    
    // Ensure directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    // Initialize SurrealKV (embedded)
    let db = Surreal::new::<SurrealKv>(db_path).await.map_err(|e| e.to_string())?;
    
    // Select namespace and database
    db.use_ns("crabflow").use_db("main").await.map_err(|e| e.to_string())?;
    
    DB.set(db).map_err(|_| "Database already initialized".to_string())?;
    
    Ok(())
}

pub fn get() -> &'static Surreal<surrealdb::engine::local::Db> {
    DB.get().expect("Database not initialized")
}

use crate::user_management::user::{UserDatabase, User, Group, UserSettings};

pub async fn migrate_legacy() -> Result<(), String> {
    let user_db_path = paths::get_db_path("users.json");
    
    if user_db_path.exists() {
        let content = std::fs::read_to_string(&user_db_path).map_err(|e| e.to_string())?;
        // We might fail deserialization if format changed, but assuming compatibility
        let old_db: UserDatabase = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        
        let db = get();
        
        // Migrate Users
        for user in old_db.users {
            // Use username as ID part
            let _: Option<User> = db.update(("users", &user.username)).content(user).await.ok().flatten();
        }
        
        // Migrate Groups
        for group in old_db.groups {
            let _: Option<Group> = db.update(("groups", &group.name)).content(group).await.ok().flatten();
        }
        
        // Migrate Settings
        let _: Option<UserSettings> = db.update(("settings", "main")).content(old_db.settings).await.ok().flatten();
        
        // Rename legacy file
        let backup_path = paths::get_db_path("users.json.migrated");
        if let Err(e) = std::fs::rename(&user_db_path, &backup_path) {
            return Err(format!("Failed to rename legacy DB: {}", e));
        }
        
        crate::sysmodules::logging::log_info("Migrated legacy users.json to SurrealDB");
    }
    
    Ok(())
}

