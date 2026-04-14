// Previene que se abra una terminal en Windows al iniciar la app
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Inicializa el logger con nivel INFO por defecto
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "entropy_client_lib=info,tauri=info".into()),
        )
        .init();

    tracing::info!("Starting Entropy client");
    entropy_client_lib::run();
}
