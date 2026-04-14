use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::AppState;

/// Respuesta exacta que devuelve el BFF en GET /servers
#[derive(Debug, Deserialize)]
struct BffConnectionInfo {
    node_id: String,
    address: String,
    voice_port: u16,
    signaling_port: u16,
    region: String,
    node_type: String,
    status: String,
    load_percent: u8,
    estimated_latency_ms: Option<u32>,
}

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
    let url = format!("{}/servers", state.bff_url);
    tracing::info!("Fetching server list from BFF: {}", url);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("BFF returned {status}: {body}"));
    }

    let nodes: Vec<BffConnectionInfo> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse BFF response: {e}"))?;

    let servers = nodes
        .into_iter()
        .map(|n| ServerInfo {
            id: n.node_id.clone(),
            name: format!("{} — {}", n.node_type, n.region),
            region: n.region,
            player_count: n.load_percent as u32,
            max_players: 100,
            ping_ms: n.estimated_latency_ms.unwrap_or(0),
            server_type: n.node_type,
            ip: n.address,
            port_udp: n.voice_port,
            port_quic: n.signaling_port,
        })
        .collect();

    Ok(servers)
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
