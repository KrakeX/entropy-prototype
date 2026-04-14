use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::atomic::Ordering;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSettings {
    pub competition_mode: bool,
    pub mic_muted: bool,
    pub audio_muted: bool,
    pub input_device_id: Option<String>,
    pub output_device_id: Option<String>,
    pub voice_activation: bool,
    pub voice_activation_threshold: f32,
    pub bff_url: String,
}

/// Activa o desactiva el Modo Competición.
/// En Rust, esto debería reducir polling, cerrar conexiones no esenciales, minimizar CPU.
/// Por ahora es solo un flag booleano.
#[tauri::command]
pub async fn toggle_competition_mode(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let was_active = state.competition_mode.fetch_xor(true, Ordering::Relaxed);
    let now_active = !was_active;
    tracing::info!("Competition mode: {}", now_active);
    Ok(now_active)
}

/// Devuelve la configuración actual del cliente al frontend.
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<ClientSettings, String> {
    Ok(ClientSettings {
        competition_mode: state.competition_mode.load(Ordering::Relaxed),
        mic_muted: state.mic_muted.load(Ordering::Relaxed),
        audio_muted: state.audio_muted.load(Ordering::Relaxed),
        input_device_id: None,
        output_device_id: None,
        voice_activation: false,
        voice_activation_threshold: 0.02,
        bff_url: state.bff_url.clone(),
    })
}

/// Actualiza la configuración del cliente y la persiste en disco.
#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: ClientSettings,
) -> Result<(), String> {
    tracing::info!("Updating settings");
    state.competition_mode.store(settings.competition_mode, Ordering::Relaxed);
    state.mic_muted.store(settings.mic_muted, Ordering::Relaxed);
    state.audio_muted.store(settings.audio_muted, Ordering::Relaxed);
    // TODO: persist to disk using tauri's path resolver
    Ok(())
}
