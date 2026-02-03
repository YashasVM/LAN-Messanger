use crate::{Message, Peer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

const DISCOVERY_PORT: u16 = 45677;
const BROADCAST_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscoveryPacket {
    id: String,
    name: String,
    port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkMessage {
    msg_type: String, // "text" or "file"
    payload: Message,
}

pub fn start_discovery(
    my_id: String,
    my_name: String,
    my_port: u16,
    peers: Arc<Mutex<HashMap<String, Peer>>>,
) {
    // Get local IP
    let local_ip = local_ip_address::local_ip().unwrap_or("127.0.0.1".parse().unwrap());

    // Start broadcast thread
    let my_id_clone = my_id.clone();
    let my_name_clone = my_name.clone();
    thread::spawn(move || {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to bind UDP socket for broadcast: {}", e);
                return;
            }
        };
        socket.set_broadcast(true).unwrap();

        loop {
            let packet = DiscoveryPacket {
                id: my_id_clone.clone(),
                name: my_name_clone.clone(),
                port: my_port,
            };

            let data = serde_json::to_vec(&packet).unwrap();

            // Broadcast to subnet
            let broadcast_addr = format!("255.255.255.255:{}", DISCOVERY_PORT);
            let _ = socket.send_to(&data, &broadcast_addr);

            thread::sleep(BROADCAST_INTERVAL);
        }
    });

    // Start listener thread
    thread::spawn(move || {
        let socket = match UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT)) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to bind UDP socket for discovery: {}", e);
                return;
            }
        };

        let mut buf = [0u8; 1024];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((len, addr)) => {
                    if let Ok(packet) = serde_json::from_slice::<DiscoveryPacket>(&buf[..len]) {
                        // Don't add ourselves
                        if packet.id != my_id {
                            let peer = Peer {
                                id: packet.id.clone(),
                                name: packet.name,
                                ip: addr.ip().to_string(),
                                port: packet.port,
                                last_seen: chrono::Utc::now().timestamp(),
                            };

                            let mut peers_lock = peers.lock().unwrap();
                            peers_lock.insert(packet.id, peer);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Discovery recv error: {}", e);
                }
            }
        }
    });
}

pub fn start_message_server(
    port: u16,
    messages: Arc<Mutex<Vec<Message>>>,
    app_handle: AppHandle,
) {
    let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to bind TCP listener: {}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let messages = messages.clone();
                let app_handle = app_handle.clone();

                thread::spawn(move || {
                    let mut buffer = Vec::new();
                    let mut temp_buf = [0u8; 4096];

                    loop {
                        match stream.read(&mut temp_buf) {
                            Ok(0) => break,
                            Ok(n) => buffer.extend_from_slice(&temp_buf[..n]),
                            Err(_) => break,
                        }
                    }

                    if let Ok(net_msg) = serde_json::from_slice::<NetworkMessage>(&buffer) {
                        let msg = net_msg.payload;

                        // Store the message
                        {
                            let mut msgs = messages.lock().unwrap();
                            msgs.push(msg.clone());
                        }

                        // Emit event to frontend
                        let _ = app_handle.emit("message_received", &msg);

                        // Show notification
                        let _ = app_handle.emit("show_notification", serde_json::json!({
                            "title": format!("Message from {}", msg.from_name),
                            "body": if msg.is_file {
                                format!("Sent a file: {}", msg.file_name.unwrap_or_default())
                            } else {
                                msg.content.chars().take(50).collect::<String>()
                            }
                        }));
                    }
                });
            }
            Err(e) => {
                log::error!("TCP accept error: {}", e);
            }
        }
    }
}

pub async fn send_message_to_peer(ip: &str, port: u16, msg: &Message) -> Result<(), String> {
    let addr = format!("{}:{}", ip, port);

    let mut stream = TcpStream::connect_timeout(
        &addr.parse::<SocketAddr>().map_err(|e| e.to_string())?,
        Duration::from_secs(5),
    )
    .map_err(|e| format!("Failed to connect: {}", e))?;

    let net_msg = NetworkMessage {
        msg_type: "text".to_string(),
        payload: msg.clone(),
    };

    let data = serde_json::to_vec(&net_msg).map_err(|e| e.to_string())?;
    stream.write_all(&data).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn send_file_to_peer(ip: &str, port: u16, msg: &Message) -> Result<(), String> {
    let addr = format!("{}:{}", ip, port);

    let mut stream = TcpStream::connect_timeout(
        &addr.parse::<SocketAddr>().map_err(|e| e.to_string())?,
        Duration::from_secs(30),
    )
    .map_err(|e| format!("Failed to connect: {}", e))?;

    let net_msg = NetworkMessage {
        msg_type: "file".to_string(),
        payload: msg.clone(),
    };

    let data = serde_json::to_vec(&net_msg).map_err(|e| e.to_string())?;
    stream.write_all(&data).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;

    Ok(())
}
