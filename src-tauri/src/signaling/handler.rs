//! Procesamiento de mensajes recibidos del servidor via QUIC.
//! Incluye: confirmaciones de join, mensajes de chat, presencia de usuarios.

/// Tipos de mensajes que el servidor puede enviar al cliente.
#[derive(Debug)]
#[allow(dead_code)]
pub enum ServerMessage {
    JoinConfirmed { channel_id: String, participants: Vec<String> },
    UserJoined { user_id: String, username: String },
    UserLeft { user_id: String },
    TextMessage { channel_id: String, user_id: String, content: String, timestamp: u64 },
    PeerKeyExchange { peer_id: String, public_key: Vec<u8> },
}

/// Procesa un mensaje recibido del servidor y actualiza el estado de la app.
#[allow(dead_code)]
pub async fn handle_message(_msg: ServerMessage) -> anyhow::Result<()> {
    todo!("message handling not yet implemented")
}
