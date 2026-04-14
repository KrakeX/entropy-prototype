use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::atomic::Ordering;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevices {
    pub input: Vec<AudioDevice>,
    pub output: Vec<AudioDevice>,
    pub current_input: Option<String>,
    pub current_output: Option<String>,
}

/// Inicia la captura de audio y el envío al canal de voz especificado.
#[tauri::command]
pub async fn start_voice(
    state: State<'_, AppState>,
    channel_id: String,
) -> Result<(), String> {
    tracing::info!("Starting voice in channel {}", channel_id);
    let _ = &state;
    Err("not implemented".to_string())
}

/// Detiene la captura de audio y abandona el canal de voz.
#[tauri::command]
pub async fn stop_voice(state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Stopping voice");
    let _ = &state;
    Err("not implemented".to_string())
}

/// Alterna el estado mute del micrófono. Devuelve el nuevo estado (true = muteado).
#[tauri::command]
pub async fn toggle_mute(state: State<'_, AppState>) -> Result<bool, String> {
    let was_muted = state.mic_muted.fetch_xor(true, Ordering::Relaxed);
    let now_muted = !was_muted;
    tracing::info!("Mic muted: {}", now_muted);
    Ok(now_muted)
}

/// Alterna el estado deafen (silencio de salida). Devuelve el nuevo estado.
#[tauri::command]
pub async fn toggle_deafen(state: State<'_, AppState>) -> Result<bool, String> {
    let was_deafened = state.audio_muted.fetch_xor(true, Ordering::Relaxed);
    let now_deafened = !was_deafened;
    tracing::info!("Audio deafened: {}", now_deafened);
    Ok(now_deafened)
}

/// Lista los dispositivos de audio disponibles en el sistema.
#[tauri::command]
pub async fn list_audio_devices() -> Result<AudioDevices, String> {
    tracing::info!("Listing audio devices");
    Err("not implemented".to_string())
}

/// Establece el dispositivo de audio activo (input o output).
#[tauri::command]
pub async fn set_audio_device(
    device_id: String,
    device_type: String,
) -> Result<(), String> {
    tracing::info!("Setting {} device: {}", device_type, device_id);
    Err("not implemented".to_string())
}
