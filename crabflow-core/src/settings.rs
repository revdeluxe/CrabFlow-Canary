use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub app_name: String,
    pub port: u16,
    pub enable_ui: bool,
    pub sdnc_mode: String,
    pub log_level: String,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            app_name: "CrabFlow Canary".into(),
            port: 8000,
            enable_ui: true,
            sdnc_mode: "hybrid".into(),
            log_level: "info".into(),
        }
    }

    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let mut settings = Self::default();

        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(mut partial_map) = serde_json::from_str::<Map<String, Value>>(&content) {
                    let full = serde_json::to_value(&settings).unwrap();
                    if let Some(full_map) = full.as_object() {
                        for (k, v) in full_map {
                            partial_map.entry(k.clone()).or_insert(v.clone());
                        }
                        settings =
                            serde_json::from_value(Value::Object(partial_map)).unwrap_or(settings);
                    }
                }
            }
        }

        // Write back the full config (append missing fields)
        let _ = fs::write(path, serde_json::to_string_pretty(&settings).unwrap());
        settings
    }
}
