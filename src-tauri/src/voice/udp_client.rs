//! Envío y recepción de paquetes UDP de voz al nodo SFU (Selective Forwarding Unit).
//! Protocolo: [4 bytes peer_id][4 bytes seq][N bytes opus_payload_cifrado]

/// Envía un paquete de voz cifrado al nodo.
#[allow(dead_code)]
pub async fn send_voice_packet(
    _socket: &tokio::net::UdpSocket,
    _packet: Vec<u8>,
) -> anyhow::Result<()> {
    todo!("UDP send not yet implemented")
}

/// Recibe un paquete de voz del nodo y lo devuelve sin descifrar.
#[allow(dead_code)]
pub async fn recv_voice_packet(
    _socket: &tokio::net::UdpSocket,
) -> anyhow::Result<Vec<u8>> {
    todo!("UDP recv not yet implemented")
}
