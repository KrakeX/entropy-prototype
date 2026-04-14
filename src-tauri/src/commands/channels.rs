use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantInfo {
    pub user_id: String,
    pub username: String,
    pub is_muted: bool,
    pub is_speaking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub participants: Vec<ParticipantInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId {
    pub id: String,
    pub timestamp: u64,
}

/// Une al cliente al canal de voz especificado e inicia el handshake E2E con los peers.
#[tauri::command]
pub async fn join_voice_channel(
    state: State<'_, AppState>,
    channel_id: String,
) -> Result<ChannelInfo, String> {
    tracing::info!("Joining voice channel {}", channel_id);
    let _ = &state;
    Err("not implemented".to_string())
}

/// Abandona el canal de voz actual y libera los recursos de audio.
#[tauri::command]
pub async fn leave_voice_channel(state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Leaving voice channel");
    let _ = &state;
    Err("not implemented".to_string())
}

/// Envía un mensaje de texto al canal especificado a través de la conexión QUIC.
/// El mensaje se cifra E2E antes de enviarse.
#[tauri::command]
pub async fn send_text_message(
    state: State<'_, AppState>,
    channel_id: String,
    content: String,
) -> Result<MessageId, String> {
    tracing::info!("Sending message to channel {}", channel_id);
    let _ = (&state, &content);
    Err("not implemented".to_string())
}
