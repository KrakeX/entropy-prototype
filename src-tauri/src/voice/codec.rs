//! Encoding y decoding de audio con el codec Opus.
//! Tasa de muestreo: 48kHz. Canales: 1 (mono). Frame: 20ms.

/// Codifica un frame PCM a bytes Opus comprimidos.
#[allow(dead_code)]
pub fn encode_frame(_pcm: &[f32]) -> anyhow::Result<Vec<u8>> {
    // TODO: usar audiopus para encodear
    todo!("opus encode not yet implemented")
}

/// Decodifica bytes Opus a un frame PCM.
#[allow(dead_code)]
pub fn decode_frame(_opus_data: &[u8]) -> anyhow::Result<Vec<f32>> {
    // TODO: usar audiopus para decodear
    todo!("opus decode not yet implemented")
}
