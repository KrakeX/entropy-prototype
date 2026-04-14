//! Descifrado AES-256-GCM de paquetes recibidos.

use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
use aes_gcm::aead::generic_array::GenericArray;

/// Descifra un payload cifrado con AES-256-GCM.
/// Espera formato [nonce (12 bytes) | ciphertext].
#[allow(dead_code)]
pub fn decrypt(key: &[u8; 32], data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if data.len() < 12 {
        return Err(anyhow::anyhow!("Data too short to contain nonce"));
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
}
