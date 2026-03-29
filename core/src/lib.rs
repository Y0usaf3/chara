mod db;
pub mod models;
pub mod service;
pub use db::init;
pub use dotenvy;

use chacha20poly1305::Key;
use hackclub_auth_api::HCAuth;
use std::sync::LazyLock;

pub static HCAUTH: LazyLock<HCAuth> = LazyLock::new(|| {
    HCAuth::new(
        env_required!("CLIENT_ID").as_str(),
        env_required!("CLIENT_SECRET").as_str(),
        env_required!("REDIRECT_URI").as_str(),
    )
});

pub static MASTER_KEY: LazyLock<Key> = LazyLock::new(|| {
    let key_hex = env_required!("MASTER_KEY");
    let key_bytes = hex::decode(&key_hex).expect("MASTER_KEY must be valid hex string");
    if key_bytes.len() != 32 {
        panic!("MASTER_KEY must be exactly 32 bytes (64 hex characters)");
    }
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    Key::from(key_array)
});
