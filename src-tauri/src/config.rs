use serde::{Deserialize, Serialize};

/// Configuración persistente del cliente, guardada en el directorio de datos de la app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub bff_url: String,
    pub default_input_device: Option<String>,
    pub default_output_device: Option<String>,
    pub competition_mode: bool,
    pub voice_activation: bool,
    pub voice_activation_threshold: f32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            bff_url: "http://localhost:8080".to_string(),
            default_input_device: None,
            default_output_device: None,
            competition_mode: false,
            voice_activation: false,
            voice_activation_threshold: 0.02,
        }
    }
}
