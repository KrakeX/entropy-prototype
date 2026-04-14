//! Mezcla de streams de audio de múltiples participantes.
//! Suma los frames PCM de cada peer y normaliza para evitar clipping.

/// Mezcla múltiples frames PCM en uno solo.
#[allow(dead_code)]
pub fn mix_frames(frames: &[Vec<f32>]) -> Vec<f32> {
    // TODO: suma y normalización de frames
    let _ = frames;
    vec![]
}
