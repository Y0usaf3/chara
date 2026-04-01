use app::*;
use axum::Router;
use axum::extract::FromRef;
use axum::routing::get;
use axum_extra::extract::cookie::Key;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use std::net::SocketAddr;

mod oauth;

#[tokio::main]
async fn main() {
    charac::init().await;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let state = AppState {
        leptos_options,
        // NOTE: we only generate new keys for testing, soon we will have to store the key
        // somewhere else
        key: Key::generate(),
    };

    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/callback", get(oauth::oauth))
        .route("/auth", get(oauth::redirect_to_oauth))
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let state = state.clone();
                move || provide_context(state.clone())
            },
            {
                let leptos_options = state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>({
            let options = state.leptos_options.clone();
            move |_| shell(options.clone())
        }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
