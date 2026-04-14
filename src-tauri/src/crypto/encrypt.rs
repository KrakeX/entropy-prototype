//! Cifrado AES-256-GCM de paquetes de voz.
//! Cada paquete incluye un nonce único de 96 bits generado aleatoriamente.

use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, OsRng, rand_core::RngCore}};
use aes_gcm::aead::generic_array::GenericArray;

/// Cifra un payload con AES-256-GCM.
/// Devuelve [nonce (12 bytes) | ciphertext].
#[allow(dead_code)]
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}
