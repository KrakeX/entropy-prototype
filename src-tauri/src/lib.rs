mod commands;
mod config;
mod crypto;
mod signaling;
mod state;
mod voice;

use commands::{audio, channels, connection, servers, settings};
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new("http://localhost:8080".to_string()))
        .invoke_handler(tauri::generate_handler![
            // Connection
            connection::connect_to_node,
            connection::disconnect,
            connection::get_connection_status,
            // Audio
            audio::start_voice,
            audio::stop_voice,
            audio::toggle_mute,
            audio::toggle_deafen,
            audio::list_audio_devices,
            audio::set_audio_device,
            // Channels
            channels::join_voice_channel,
            channels::leave_voice_channel,
            channels::send_text_message,
            // Servers
            servers::fetch_server_list,
            servers::fetch_server_details,
            // Settings
            settings::toggle_competition_mode,
            settings::get_settings,
            settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
