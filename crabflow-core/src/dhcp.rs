use log::{error, info};
use std::net::{Ipv4Addr, UdpSocket};

// src/dhcp.rs

/// A module for handling DHCP (Dynamic Host Configuration Protocol) functionality.

pub mod dhcp {
    use super::*;

    /// Represents a DHCP message type.
    #[derive(Debug, PartialEq)]
    pub enum DhcpMessageType {
        Discover,
        Offer,
        Request,
        Ack,
        Nak,
        Release,
        Inform,
        Unknown,
    }

    /// Represents a DHCP message.
    #[derive(Debug)]
    pub struct DhcpMessage {
        pub message_type: DhcpMessageType,
        pub transaction_id: u32,
        pub client_mac: [u8; 6],
        pub options: std::collections::HashMap<u8, Vec<u8>>,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C, packed)]
    pub struct DhcpPacket {
        pub op: u8,
        pub htype: u8,
        pub hlen: u8,
        pub hops: u8,
        pub xid: [u8; 4],
        pub secs: [u8; 2],
        pub flags: [u8; 2],
        pub ciaddr: [u8; 4],
        pub yiaddr: [u8; 4],
        pub siaddr: [u8; 4],
        pub giaddr: [u8; 4],
        pub chaddr: [u8; 16],
        pub sname: [u8; 64],
        pub file: [u8; 128],
        pub magic_cookie: [u8; 4],
        pub options: [u8; 308],
    }

    impl DhcpMessage {
        /// Creates a new DHCP message.
        pub fn new(
            message_type: DhcpMessageType,
            transaction_id: u32,
            client_mac: [u8; 6],
        ) -> Self {
            Self {
                message_type,
                transaction_id,
                client_mac,
                options: std::collections::HashMap::new(),
            }
        }
    }

    pub fn deserialize_message(data: &[u8]) -> Result<DhcpMessage, String> {
        if data.len() < 240 {
            return Err("Packet too small".to_string());
        }

        let packet: &DhcpPacket = unsafe { &*(data.as_ptr() as *const DhcpPacket) };

        if packet.magic_cookie != [0x63, 0x82, 0x53, 0x63] {
            return Err("Invalid magic cookie".to_string());
        }

        let transaction_id = u32::from_be_bytes(packet.xid);
        let mut client_mac = [0u8; 6];
        client_mac.copy_from_slice(&packet.chaddr[0..6]);

        let mut options = std::collections::HashMap::new();
        let mut i = 0;
        while i < packet.options.len() {
            let option_code = packet.options[i];
            if option_code == 255 {
                // End option
                break;
            }
            if i + 1 >= packet.options.len() {
                break;
            }
            let option_len = packet.options[i + 1] as usize;
            if i + 2 + option_len > packet.options.len() {
                break;
            }
            let option_data = packet.options[i + 2..i + 2 + option_len].to_vec();
            options.insert(option_code, option_data);
            i += 2 + option_len;
        }

        let message_type = if let Some(option_data) = options.get(&53) {
            if option_data.len() == 1 {
                match option_data[0] {
                    1 => DhcpMessageType::Discover,
                    2 => DhcpMessageType::Offer,
                    3 => DhcpMessageType::Request,
                    5 => DhcpMessageType::Ack,
                    6 => DhcpMessageType::Nak,
                    7 => DhcpMessageType::Release,
                    8 => DhcpMessageType::Inform,
                    _ => DhcpMessageType::Unknown,
                }
            } else {
                DhcpMessageType::Unknown
            }
        } else {
            DhcpMessageType::Unknown
        };

        Ok(DhcpMessage {
            message_type,
            transaction_id,
            client_mac,
            options,
        })
    }

    pub fn serialize_message(message: &DhcpMessage) -> Vec<u8> {
        // Placeholder implementation
        vec![]
    }

    pub struct DhcpServer {
        socket: UdpSocket,
    }

    impl DhcpServer {
        pub fn new() -> std::io::Result<Self> {
            let socket = UdpSocket::bind("0.0.0.0:67")?;
            Ok(Self { socket })
        }

        pub fn listen(&self) {
            let mut buf = [0; 1500];
            loop {
                match self.socket.recv_from(&mut buf) {
                    Ok((size, src)) => {
                        info!("Received {} bytes from {}", size, src);
                        match deserialize_message(&buf[..size]) {
                            Ok(message) => {
                                info!("Received DHCP message: {:?}", message);
                            }
                            Err(e) => {
                                error!("Failed to deserialize DHCP message: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to receive data: {}", e);
                    }
                }
            }
        }
    }
}
