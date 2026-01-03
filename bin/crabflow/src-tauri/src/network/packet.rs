// src-tauri/src/network/packet.rs
use serde::{Serialize, Deserialize};
use std::net::UdpSocket;
use crate::sysmodules::logging;

#[derive(Serialize, Deserialize, Debug)]
pub struct PacketData {
    pub source: String,
    pub destination: String,
    pub payload: String, // Hex or text
}

/// Send a packet (UDP for simplicity in this SDN simulation)
#[tauri::command]
pub fn send_packet(target: String, payload: String) -> Result<(), String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    socket.connect(target).map_err(|e| e.to_string())?;
    socket.send(payload.as_bytes()).map_err(|e| e.to_string())?;
    logging::log_info(&format!("Sent packet to {}: {}", payload, payload)); // Fixed log message
    Ok(())
}

use std::thread;

/// Start packet listener in a background thread
#[tauri::command]
pub fn start_packet_listener(port: u16) {
    thread::spawn(move || {
        if let Err(e) = receive_packets(port) {
            logging::log_error(&format!("Packet listener failed: {}", e));
        }
    });
}

/// Receive packets (Blocking, should be run in thread)
/// In a real SDN, this would be a listener on a specific interface
pub fn receive_packets(port: u16) -> Result<(), String> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).map_err(|e| e.to_string())?;
    logging::log_info(&format!("Listening for packets on port {}", port));
    
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let msg = String::from_utf8_lossy(&buf[..amt]);
                logging::log_info(&format!("Received packet from {}: {}", src, msg));
                // Here we could "give" the packet to another component
            }
            Err(e) => logging::log_error(&format!("Error receiving packet: {}", e)),
        }
    }
}
