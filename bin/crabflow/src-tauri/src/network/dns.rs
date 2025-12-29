// src-tauri/src/dns.rs

use serde::{Serialize, Deserialize};
use crate::{fetch, post, logging};

#[derive(Serialize)]
pub struct DnsRecord {
    pub name: String,
    pub rtype: String, // "A","AAAA","CNAME","MX"
    pub value: String,
    pub ttl: u32,
}

#[derive(Deserialize)]
pub struct DnsRecordInput {
    pub name: String,
    pub rtype: String,
    pub value: String,
    pub ttl: u32,
}

/// List all DNS records
pub fn list_records() -> Vec<DnsRecord> {
    match fetch::read_file("dns.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => vec![],
    }
}

/// Add a DNS record
pub fn add_record(input: DnsRecordInput) -> Result<(), String> {
    let mut records = list_records();
    records.push(DnsRecord {
        name: input.name,
        rtype: input.rtype,
        value: input.value,
        ttl: input.ttl,
    });

    let serialized = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
    post::write_file("dns.json".into(), serialized)?;
    logging::log_event("system".into(), "add_record".into(), input.name);
    Ok(())
}

/// Remove a DNS record
pub fn remove_record(name: String, rtype: String) -> Result<(), String> {
    let mut records = list_records();
    records.retain(|r| !(r.name == name && r.rtype == rtype));

    let serialized = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
    post::write_file("dns.json".into(), serialized)?;
    logging::log_event("system".into(), "remove_record".into(), name);
    Ok(())
}
