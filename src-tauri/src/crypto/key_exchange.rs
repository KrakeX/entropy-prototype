//! Key exchange X25519 ECDH entre peers.
//! El servidor actúa como relay para intercambiar public keys.
//! El shared secret nunca llega al servidor.

use x25519_dalek::{EphemeralSecret, PublicKey};
use rand::rngs::OsRng;

/// Genera un par de claves efímeras para el key exchange.
#[allow(dead_code)]
pub fn generate_keypair() -> (EphemeralSecret, PublicKey) {
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    (secret, public)
}

/// Deriva el shared secret a partir de nuestra clave privada y la clave pública del peer.
#[allow(dead_code)]
pub fn derive_shared_secret(
    our_secret: EphemeralSecret,
    their_public: PublicKey,
) -> Vec<u8> {
    let shared = our_secret.diffie_hellman(&their_public);
    shared.as_bytes().to_vec()
}
