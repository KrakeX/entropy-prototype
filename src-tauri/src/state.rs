use std::collections::HashMap;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::{Mutex, RwLock};
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
    /// Metadatos de la conexión activa al nodo (None si desconectado)
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
    /// Objeto de conexión QUIC activo (None si desconectado)
    pub quic_connection: Arc<Mutex<Option<quinn::Connection>>>,
    /// Stream de envío de señalización QUIC (acceso exclusivo para enviar mensajes)
    pub quic_send: Arc<Mutex<Option<quinn::SendStream>>>,
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
            quic_connection: Arc::new(Mutex::new(None)),
            quic_send: Arc::new(Mutex::new(None)),
        }
    }
}
