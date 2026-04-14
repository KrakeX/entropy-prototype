use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub node_id: String,
    pub ip: String,
    pub port_udp: u16,
    pub port_quic: u16,
    pub latency_ms: u32,
    pub encryption_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub connected: bool,
    pub node_id: Option<String>,
    pub latency_ms: Option<u32>,
    pub encryption_active: bool,
    pub competition_mode: bool,
}

/// Conecta el cliente a un nodo Entropy.
/// Establece sesión QUIC para señalización y prepara el socket UDP para voz.
#[tauri::command]
pub async fn connect_to_node(
    state: State<'_, AppState>,
    ip: String,
    port_udp: u16,
    port_quic: u16,
) -> Result<ConnectionInfo, String> {
    tracing::info!("Connecting to node {}:{}/{}", ip, port_udp, port_quic);
    let _ = &state;
    Err("not implemented".to_string())
}

/// Desconecta del nodo actual, libera recursos de voz y cierra la sesión QUIC.
#[tauri::command]
pub async fn disconnect(state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Disconnecting from current node");
    let _ = &state;
    Err("not implemented".to_string())
}

/// Devuelve el estado actual de la conexión al frontend.
#[tauri::command]
pub async fn get_connection_status(
    state: State<'_, AppState>,
) -> Result<ConnectionStatus, String> {
    use std::sync::atomic::Ordering;
    let connection = state.current_connection.read().await;
    Ok(ConnectionStatus {
        connected: connection.is_some(),
        node_id: connection.as_ref().map(|c| c.node_id.clone()),
        latency_ms: None,
        encryption_active: connection.is_some(),
        competition_mode: state.competition_mode.load(Ordering::Relaxed),
    })
}
