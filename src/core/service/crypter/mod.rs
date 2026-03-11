use crate::core::service::errors::EncryptionError;
use crate::MASTER_KEY;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

pub async fn encrypt_token(token: &str) -> Result<Vec<u8>, EncryptionError> {
    let cipher = ChaCha20Poly1305::new(&MASTER_KEY);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, token.as_ref())
        .map_err(|_| EncryptionError::EncryptionFailed)?;

    let mut encrypted = nonce.to_vec();
    encrypted.extend_from_slice(&ciphertext);

    Ok(encrypted)
}

pub async fn decrypt_token(encrypted: &[u8]) -> Result<String, EncryptionError> {
    if encrypted.len() < 12 {
        return Err(EncryptionError::DecryptionFailed);
    }

    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = ChaCha20Poly1305::new(&MASTER_KEY);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| EncryptionError::InvalidNonce)?;

    String::from_utf8(plaintext).map_err(|_| EncryptionError::DecryptionFailed)
}
