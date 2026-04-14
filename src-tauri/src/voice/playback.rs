//! Reproducción de audio recibido usando cpal.

/// Inicia el stream de reproducción. Recibe frames PCM decodificados por el channel.
#[allow(dead_code)]
pub async fn start_playback(
    _rx: tokio::sync::mpsc::Receiver<Vec<f32>>,
) -> anyhow::Result<()> {
    // TODO: usar cpal para abrir el stream de salida
    // TODO: reproducir frames PCM recibidos via rx
    todo!("audio playback not yet implemented")
}
