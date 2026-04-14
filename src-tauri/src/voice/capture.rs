//! Captura de audio del micrófono usando cpal.
//! El audio capturado se envía al codec Opus para compresión antes de transmitir.

/// Inicia la captura de audio del dispositivo de entrada por defecto.
/// Los frames de audio se envían por el channel proporcionado.
#[allow(dead_code)]
pub async fn start_capture(
    _tx: tokio::sync::mpsc::Sender<Vec<f32>>,
) -> anyhow::Result<()> {
    // TODO: usar cpal para abrir el stream de entrada
    // TODO: enviar frames PCM al encoder Opus via tx
    todo!("audio capture not yet implemented")
}

/// Detiene la captura de audio y libera el dispositivo.
#[allow(dead_code)]
pub async fn stop_capture() -> anyhow::Result<()> {
    todo!("stop capture not yet implemented")
}
