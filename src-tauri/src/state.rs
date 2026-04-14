use std::collections::HashMap;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Identificador de un canal (string UUID)
pub type ChannelId = String;

/// Representa una conexión activa a un nodo Entropy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnection {
    pub node_id: String,
    pub ip: String,
    pub port_udp: u16,
    pub port_quic: u16,
}

/// Estado global de la aplicación compartido entre todos los comandos Tauri.
/// Envuelto en Arc para ser compartido entre hilos.
pub struct AppState {
    /// Conexión activa al nodo (None si desconectado)
    pub current_connection: Arc<RwLock<Option<NodeConnection>>>,
    /// Canal de voz activo (None si no está en un canal)
    pub current_voice_channel: Arc<RwLock<Option<ChannelId>>>,
    /// Modo Competición activo/inactivo
    pub competition_mode: Arc<AtomicBool>,
    /// Micrófono muteado
    pub mic_muted: Arc<AtomicBool>,
    /// Audio de salida muteado (deafen)
    pub audio_muted: Arc<AtomicBool>,
    /// URL del BFF (Backend For Frontend) para API REST
    pub bff_url: String,
    /// Claves compartidas por peer: peer_id -> shared_secret (X25519)
    pub peer_keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl AppState {
    pub fn new(bff_url: String) -> Self {
        Self {
            current_connection: Arc::new(RwLock::new(None)),
            current_voice_channel: Arc::new(RwLock::new(None)),
            competition_mode: Arc::new(AtomicBool::new(false)),
            mic_muted: Arc::new(AtomicBool::new(false)),
            audio_muted: Arc::new(AtomicBool::new(false)),
            bff_url,
            peer_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
