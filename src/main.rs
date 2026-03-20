// Source - https://stackoverflow.com/a/25877389
// Posted by Arjan, modified by community. See post 'Timeline' for change history
// Retrieved 2026-01-26, License - CC BY-SA 4.0

#![allow(unexpected_cfgs)]

#[cfg(test)]
mod test;

mod app;
mod core;
use crate::app::ui::App;
use crate::{app::ui::shell, core::db};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chacha20poly1305::Key;
use dotenv::dotenv;
use hackclub_auth_api::HCAuth;
use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_axum::generate_route_list;
use leptos_axum::LeptosRoutes;
use std::sync::LazyLock;

pub static HCAUTH: LazyLock<HCAuth> = LazyLock::new(|| {
    HCAuth::new(
        dotenv!("CLIENT_ID"),
        dotenv!("CLIENT_SECRET"),
        dotenv!("REDIRECT_URI"),
    )
});

pub static MASTER_KEY: LazyLock<Key> = LazyLock::new(|| {
    dotenv().ok();

    let key_hex = std::env::var("MASTER_KEY").expect("MASTER_KEY environment variable not set");

    let key_bytes = hex::decode(&key_hex).expect("MASTER_KEY must be valid hex string");

    if key_bytes.len() != 32 {
        panic!("MASTER_KEY must be exactly 32 bytes (64 hex characters)");
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    Key::from(key_array)
});

#[macro_use]
extern crate bitmask;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    db::init().await;
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let app = Router::new().leptos_routes(&leptos_options, routes, {
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9898")
        .await
        .unwrap();
    axum::serve(listener, app);
}
