use crate::core::service::user::UserServiceError;
use crate::MASTER_KEY;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

pub async fn encrypt_token(token: &str) -> Result<Vec<u8>, UserServiceError> {
    let cipher = ChaCha20Poly1305::new(&MASTER_KEY);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, token.as_ref())
        .map_err(|_| UserServiceError::EncryptionError)?;

    let mut encrypted = nonce.to_vec();
    encrypted.extend_from_slice(&ciphertext);

    Ok(encrypted)
}

pub async fn decrypt_token(encrypted: &[u8]) -> Result<String, UserServiceError> {
    if encrypted.len() < 12 {
        return Err(UserServiceError::EncryptionError);
    }

    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = ChaCha20Poly1305::new(&MASTER_KEY);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| UserServiceError::EncryptionError)?;

    String::from_utf8(plaintext).map_err(|_| UserServiceError::EncryptionError)
}
