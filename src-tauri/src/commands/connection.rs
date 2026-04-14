use entropy_protocol::messages::{ClientMessage, HelloMessage, ServerMessage};
use entropy_protocol::types::UserId;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::signaling::quic_client;
use crate::state::{AppState, NodeConnection};

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
/// Establece sesión QUIC para señalización (Hello/Welcome handshake).
#[tauri::command]
pub async fn connect_to_node(
    state: State<'_, AppState>,
    ip: String,
    port_udp: u16,
    port_quic: u16,
) -> Result<ConnectionInfo, String> {
    tracing::info!("Connecting to node {}  udp={} quic={}", ip, port_udp, port_quic);

    // 1. Establecer conexión QUIC
    let connection = quic_client::connect(&ip, port_quic)
        .await
        .map_err(|e| format!("No se pudo conectar al nodo: {e}"))?;

    // 2. Abrir stream bidireccional de señalización
    let (mut send, mut recv) = connection
        .open_bi()
        .await
        .map_err(|e| format!("Error abriendo stream de señalización: {e}"))?;

    // 3. Enviar Hello — el user_id es efímero para esta sesión
    let user_id = UserId::new();
    let hello = ClientMessage::Hello(HelloMessage {
        user_id,
        protocol_version: entropy_protocol::constants::PROTOCOL_VERSION,
        session_token: "stub".to_string(),
    });

    quic_client::send_framed(&mut send, &hello)
        .await
        .map_err(|e| format!("Error enviando Hello: {e}"))?;

    tracing::debug!(user_id = %user_id, "Hello enviado al nodo");

    // 4. Leer Welcome del servidor
    let bytes = quic_client::recv_framed(&mut recv)
        .await
        .map_err(|e| format!("Error recibiendo Welcome: {e}"))?;

    let server_msg: ServerMessage = serde_json::from_slice(&bytes)
        .map_err(|e| format!("Error parseando respuesta del nodo: {e}"))?;

    let welcome = match server_msg {
        ServerMessage::Welcome(w) => w,
        ServerMessage::Error(e) => {
            return Err(format!("Nodo rechazó la conexión: {:?} — {}", e.code, e.message));
        }
        other => {
            return Err(format!("Respuesta inesperada del nodo: {other:?}"));
        }
    };

    let node_id = welcome.node_id.to_string();
    tracing::info!(
        node_id = %node_id,
        session_id = %welcome.session_id,
        "Welcome recibido — sesión establecida"
    );

    // 5. Guardar metadatos de la conexión en el estado global
    *state.current_connection.write().await = Some(NodeConnection {
        node_id: node_id.clone(),
        ip: ip.clone(),
        port_udp,
        port_quic,
    });

    // 6. Guardar conexión y stream de envío para comandos futuros (join_voice_channel, etc.)
    *state.quic_connection.lock().await = Some(connection.clone());
    *state.quic_send.lock().await = Some(send);

    // 7. Spawn tarea de background para recibir mensajes del servidor
    tokio::spawn(async move {
        tracing::debug!("Iniciando loop de recepción de mensajes del nodo");
        loop {
            match quic_client::recv_framed(&mut recv).await {
                Ok(bytes) => {
                    match serde_json::from_slice::<ServerMessage>(&bytes) {
                        Ok(msg) => tracing::debug!(?msg, "mensaje recibido del nodo"),
                        Err(e) => tracing::warn!(error = %e, "mensaje del nodo malformado"),
                    }
                }
                Err(e) => {
                    tracing::info!(error = %e, "conexión con el nodo cerrada");
                    break;
                }
            }
        }
    });

    Ok(ConnectionInfo {
        node_id,
        ip,
        port_udp,
        port_quic,
        latency_ms: 0,
        encryption_status: "handshake_ok".to_string(),
    })
}

/// Desconecta del nodo actual, libera recursos de voz y cierra la sesión QUIC.
#[tauri::command]
pub async fn disconnect(state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Disconnecting from current node");

    // Cerrar stream de envío
    if let Some(mut send) = state.quic_send.lock().await.take() {
        let _ = send.finish();
    }

    // Cerrar conexión QUIC (el background recv task terminará solo al detectar el cierre)
    if let Some(conn) = state.quic_connection.lock().await.take() {
        conn.close(0u32.into(), b"client_disconnect");
    }

    // Limpiar metadatos de conexión
    *state.current_connection.write().await = None;

    tracing::info!("Desconectado del nodo");
    Ok(())
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
