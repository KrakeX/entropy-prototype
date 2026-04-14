//! Conexión QUIC al nodo Entropy para señalización y mensajes de texto.
//! Usa quinn como implementación de QUIC.

/// Establece una conexión QUIC al nodo especificado.
#[allow(dead_code)]
pub async fn connect(
    _ip: &str,
    _port: u16,
) -> anyhow::Result<()> {
    // TODO: configurar quinn endpoint con TLS
    // TODO: establecer conexión y streams bidireccionales
    todo!("QUIC connect not yet implemented")
}
