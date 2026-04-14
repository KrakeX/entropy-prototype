use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub id: String,
    pub name: String,
    pub region: String,
    pub player_count: u32,
    pub max_players: u32,
    pub ping_ms: u32,
    pub server_type: String,
    pub ip: String,
    pub port_udp: u16,
    pub port_quic: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPreview {
    pub id: String,
    pub name: String,
    pub unread_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChannelPreview {
    pub id: String,
    pub name: String,
    pub participants: Vec<String>,
    pub bitrate_kbps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetails {
    pub info: ServerInfo,
    pub channels_text: Vec<ChannelPreview>,
    pub channels_voice: Vec<VoiceChannelPreview>,
}

/// Obtiene la lista de servidores disponibles desde el BFF.
#[tauri::command]
pub async fn fetch_server_list(
    state: State<'_, AppState>,
) -> Result<Vec<ServerInfo>, String> {
    tracing::info!("Fetching server list from BFF: {}", state.bff_url);
    Err("not implemented".to_string())
}

/// Obtiene los detalles de un servidor específico, incluyendo sus canales.
#[tauri::command]
pub async fn fetch_server_details(
    state: State<'_, AppState>,
    server_id: String,
) -> Result<ServerDetails, String> {
    tracing::info!("Fetching details for server {}", server_id);
    let _ = &state;
    Err("not implemented".to_string())
}
