use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use uuid::Uuid;

mod network;
use network::{start_discovery, start_message_server, send_message_to_peer, send_file_to_peer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub last_seen: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from_id: String,
    pub from_name: String,
    pub to_id: String,
    pub content: String,
    pub timestamp: i64,
    pub is_file: bool,
    pub file_name: Option<String>,
    pub file_data: Option<String>,
}

pub struct AppState {
    pub my_id: String,
    pub my_name: String,
    pub my_port: u16,
    pub peers: Arc<Mutex<HashMap<String, Peer>>>,
    pub messages: Arc<Mutex<Vec<Message>>>,
}

#[tauri::command]
fn get_my_info(state: tauri::State<AppState>) -> (String, String) {
    (state.my_id.clone(), state.my_name.clone())
}

#[tauri::command]
fn set_my_name(name: String, state: tauri::State<AppState>) {
    let state_inner = state.inner();
    let _ = std::mem::replace(&mut *std::sync::RwLock::new(state_inner.my_name.clone()).write().unwrap(), name);
}

#[tauri::command]
fn get_peers(state: tauri::State<AppState>) -> Vec<Peer> {
    let peers = state.peers.lock().unwrap();
    let now = chrono::Utc::now().timestamp();
    peers.values()
        .filter(|p| now - p.last_seen < 30) // Only show peers seen in last 30 seconds
        .cloned()
        .collect()
}

#[tauri::command]
fn get_messages(peer_id: String, state: tauri::State<AppState>) -> Vec<Message> {
    let messages = state.messages.lock().unwrap();
    let my_id = &state.my_id;
    messages.iter()
        .filter(|m|
            (m.from_id == peer_id && m.to_id == *my_id) ||
            (m.from_id == *my_id && m.to_id == peer_id)
        )
        .cloned()
        .collect()
}

#[tauri::command]
async fn send_message(to_id: String, content: String, state: tauri::State<'_, AppState>, app: AppHandle) -> Result<Message, String> {
    let peer = {
        let peers = state.peers.lock().unwrap();
        peers.get(&to_id).cloned()
    };

    let peer = peer.ok_or("Peer not found")?;

    let msg = Message {
        id: Uuid::new_v4().to_string(),
        from_id: state.my_id.clone(),
        from_name: state.my_name.clone(),
        to_id: to_id.clone(),
        content,
        timestamp: chrono::Utc::now().timestamp_millis(),
        is_file: false,
        file_name: None,
        file_data: None,
    };

    send_message_to_peer(&peer.ip, peer.port, &msg).await?;

    {
        let mut messages = state.messages.lock().unwrap();
        messages.push(msg.clone());
    }

    let _ = app.emit("message_sent", &msg);

    Ok(msg)
}

#[tauri::command]
async fn send_file(to_id: String, file_path: String, state: tauri::State<'_, AppState>, app: AppHandle) -> Result<Message, String> {
    let peer = {
        let peers = state.peers.lock().unwrap();
        peers.get(&to_id).cloned()
    };

    let peer = peer.ok_or("Peer not found")?;

    let file_data = std::fs::read(&file_path).map_err(|e| e.to_string())?;
    let file_name = std::path::Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();

    let msg = Message {
        id: Uuid::new_v4().to_string(),
        from_id: state.my_id.clone(),
        from_name: state.my_name.clone(),
        to_id: to_id.clone(),
        content: format!("Sent file: {}", file_name),
        timestamp: chrono::Utc::now().timestamp_millis(),
        is_file: true,
        file_name: Some(file_name),
        file_data: Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &file_data)),
    };

    send_file_to_peer(&peer.ip, peer.port, &msg).await?;

    {
        let mut messages = state.messages.lock().unwrap();
        messages.push(msg.clone());
    }

    let _ = app.emit("message_sent", &msg);

    Ok(msg)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let my_id = Uuid::new_v4().to_string();
    let my_name = whoami::username();
    let my_port = 45678;

    let peers: Arc<Mutex<HashMap<String, Peer>>> = Arc::new(Mutex::new(HashMap::new()));
    let messages: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(Vec::new()));

    let state = AppState {
        my_id: my_id.clone(),
        my_name: my_name.clone(),
        my_port,
        peers: peers.clone(),
        messages: messages.clone(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_my_info,
            set_my_name,
            get_peers,
            get_messages,
            send_message,
            send_file
        ])
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_handle = app.handle().clone();
            let peers_clone = peers.clone();
            let messages_clone = messages.clone();
            let my_id_clone = my_id.clone();
            let my_name_clone = my_name.clone();

            // Start discovery service
            thread::spawn(move || {
                start_discovery(my_id_clone.clone(), my_name_clone.clone(), my_port, peers_clone.clone());
            });

            // Start message server
            let app_handle_clone = app_handle.clone();
            thread::spawn(move || {
                start_message_server(my_port, messages_clone, app_handle_clone);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
