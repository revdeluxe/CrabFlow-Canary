// src-tauri/src/network/firewall.rs
use serde::{Serialize, Deserialize};
use crate::sysmodules::{fetch, post, logging, paths};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FirewallRule {
    pub id: String,
    pub name: String,
    pub port: u16,
    pub protocol: String, // TCP, UDP, ANY
    pub action: String, // ALLOW, DENY
    pub direction: String, // INBOUND, OUTBOUND
}

#[derive(Deserialize)]
pub struct FirewallRuleInput {
    pub name: String,
    pub port: u16,
    pub protocol: String,
    pub action: String,
    pub direction: String,
}

lazy_static! {
    static ref FIREWALL_RULES: Mutex<Vec<FirewallRule>> = Mutex::new(Vec::new());
}

fn get_firewall_file() -> String {
    paths::get_config_path("firewall.json").to_string_lossy().to_string()
}

pub fn init_firewall() {
    match fetch::read_file(&get_firewall_file()) {
        Ok(data) => {
            let rules: Vec<FirewallRule> = serde_json::from_str(&data).unwrap_or_default();
            let mut cache = FIREWALL_RULES.lock().unwrap();
            *cache = rules;
            logging::log_info("Firewall rules loaded.");
        },
        Err(_) => {
            logging::log_info("No firewall config found. Starting empty.");
        }
    }
}

fn save_rules() -> Result<(), String> {
    let rules = FIREWALL_RULES.lock().unwrap();
    let serialized = serde_json::to_string_pretty(&*rules).map_err(|e| e.to_string())?;
    post::write_file(&get_firewall_file(), &serialized)
}

#[tauri::command]
pub fn list_firewall_rules() -> Vec<FirewallRule> {
    let rules = FIREWALL_RULES.lock().unwrap();
    rules.clone()
}

#[tauri::command]
pub fn add_firewall_rule(input: FirewallRuleInput) -> Result<(), String> {
    let mut rules = FIREWALL_RULES.lock().unwrap();
    
    let new_rule = FirewallRule {
        id: Uuid::new_v4().to_string(),
        name: input.name,
        port: input.port,
        protocol: input.protocol,
        action: input.action,
        direction: input.direction,
    };
    
    rules.push(new_rule.clone());
    drop(rules); // Unlock before saving
    save_rules()?;
    
    logging::log_event("firewall".into(), "add_rule".into(), new_rule.name);
    Ok(())
}

#[tauri::command]
pub fn update_firewall_rule(id: String, input: FirewallRuleInput) -> Result<(), String> {
    let mut rules = FIREWALL_RULES.lock().unwrap();
    
    if let Some(rule) = rules.iter_mut().find(|r| r.id == id) {
        rule.name = input.name;
        rule.port = input.port;
        rule.protocol = input.protocol;
        rule.action = input.action;
        rule.direction = input.direction;
    } else {
        return Err("Rule not found".to_string());
    }
    
    drop(rules);
    save_rules()?;
    logging::log_event("firewall".into(), "update_rule".into(), id);
    Ok(())
}

#[tauri::command]
pub fn delete_firewall_rule(id: String) -> Result<(), String> {
    let mut rules = FIREWALL_RULES.lock().unwrap();
    let len_before = rules.len();
    rules.retain(|r| r.id != id);
    
    if rules.len() == len_before {
        return Err("Rule not found".to_string());
    }
    
    drop(rules);
    save_rules()?;
    logging::log_event("firewall".into(), "delete_rule".into(), id);
    Ok(())
}
// Firewall management module for CrabFlow