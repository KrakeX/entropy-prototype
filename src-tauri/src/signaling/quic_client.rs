//! Cliente QUIC para conectarse al nodo Entropy de señalización.
//!
//! El servidor usa un certificado TLS self-signed generado con rcgen.
//! Para el cliente de desarrollo usamos un verificador que acepta cualquier cert.
//! En producción se cargaría el certificado del nodo o se usaría una CA propia.
//!
//! Protocolo de framing: [u32 BE: longitud del mensaje][bytes JSON del mensaje]
//! Esto es el mismo framing que usa el servidor en entropy-node.

use std::sync::Arc;

use anyhow::{Context, Result};
use quinn::{ClientConfig, Connection, Endpoint, RecvStream, SendStream};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, Error as TlsError, SignatureScheme};

const ALPN_ENTROPY: &[u8] = b"entropy-signaling/0.1";

// ─── Verificador TLS no-verificador ──────────────────────────────────────────

/// Acepta cualquier certificado TLS del servidor.
/// Solo para desarrollo — el servidor genera certs self-signed con rcgen.
#[derive(Debug)]
struct NoVerifier;

impl ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, TlsError> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, TlsError> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, TlsError> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        // Todos los esquemas que soporta ring
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::ED25519,
        ]
    }
}

// ─── Conexión ────────────────────────────────────────────────────────────────

/// Establece una conexión QUIC al nodo de señalización Entropy.
///
/// Retorna la `Connection` de quinn lista para abrir streams bidireccionales.
/// El handshake Hello/Welcome se realiza en `connect_to_node` una vez que
/// se tenga la conexión.
pub async fn connect(ip: &str, port: u16) -> Result<Connection> {
    // Instalar ring como crypto provider (falla silenciosamente si ya está instalado)
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Construir TLS config con verificador permisivo (self-signed)
    let mut tls_config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerifier))
        .with_no_client_auth();

    tls_config.alpn_protocols = vec![ALPN_ENTROPY.to_vec()];

    let quic_client_config = quinn::crypto::rustls::QuicClientConfig::try_from(tls_config)
        .context("error convirtiendo rustls config a QuicClientConfig")?;

    let client_config = ClientConfig::new(Arc::new(quic_client_config));

    // Endpoint local en un puerto efímero
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)
        .context("error creando endpoint QUIC local")?;
    endpoint.set_default_client_config(client_config);

    let server_addr: std::net::SocketAddr = format!("{ip}:{port}")
        .parse()
        .with_context(|| format!("dirección de nodo inválida: {ip}:{port}"))?;

    // El server_name es solo para SNI — no se verifica porque usamos NoVerifier
    let connection = endpoint
        .connect(server_addr, "entropy-node")
        .context("error iniciando conexión QUIC")?
        .await
        .context("error completando handshake QUIC (¿está el nodo corriendo?)")?;

    tracing::info!(
        remote = %connection.remote_address(),
        "conexión QUIC establecida con el nodo"
    );

    Ok(connection)
}

// ─── Framing ─────────────────────────────────────────────────────────────────

/// Envía un mensaje serializable con framing [u32 BE longitud][JSON] al stream.
pub async fn send_framed<T: serde::Serialize>(send: &mut SendStream, msg: &T) -> Result<()> {
    let bytes = serde_json::to_vec(msg).context("error serializando mensaje")?;
    let len = u32::try_from(bytes.len()).context("mensaje demasiado grande")?;

    send.write_all(&len.to_be_bytes())
        .await
        .context("error enviando longitud del mensaje")?;

    send.write_all(&bytes)
        .await
        .context("error enviando payload del mensaje")?;

    Ok(())
}

/// Lee un mensaje con framing [u32 BE longitud][JSON] del stream.
pub async fn recv_framed(recv: &mut RecvStream) -> Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    recv.read_exact(&mut len_buf)
        .await
        .context("error leyendo longitud del mensaje")?;

    let len = u32::from_be_bytes(len_buf) as usize;

    if len > entropy_protocol::constants::MAX_SIGNALING_MESSAGE_SIZE {
        anyhow::bail!("mensaje demasiado grande: {len} bytes");
    }

    let mut buf = vec![0u8; len];
    recv.read_exact(&mut buf)
        .await
        .context("error leyendo payload del mensaje")?;

    Ok(buf)
}
