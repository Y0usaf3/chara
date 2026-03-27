use app::*;
use axum::extract::ConnectInfo;
use axum::extract::Query;
use axum::routing::get;
use axum::{Router, http::StatusCode, response::Redirect};
use axum_extra::TypedHeader;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::SameSite;
use charac::HCAUTH;
use charac::service::user::UserService;
use headers::UserAgent;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use serde::Deserialize;
use std::net::SocketAddr;

// NOTE: NEVER, NEVER, use axum extra again, NEVER!
// NOTE: oh actually u can use it, but ONLY IN THE SERVER WORKSPACE!

#[derive(Deserialize)]
struct Code {
    code: String,
}

async fn oauth(
    Query(params): Query<Code>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    user_agent: Option<TypedHeader<UserAgent>>, // Assuming 'code' comes from a query param, adjust as needed
) -> Result<(CookieJar, Redirect), StatusCode> {
    println!("start");

    let user_agent = user_agent
        .map(|ua| ua.to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    println!("exchanging code");

    let auth_res = HCAUTH
        .exchange_code(params.code)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let access_token = auth_res.access_token.ok_or(StatusCode::UNAUTHORIZED)?;
    println!("registering");

    let service = UserService::register(access_token).await.map_err(|e| {
        eprintln!("Registration error: {:?}", e);

        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    println!("creating token");

    let session_token = service
        .create_session(addr.ip().to_string(), user_agent)
        .await
        .ok()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("creating yummy cookie");

    let cookie = Cookie::build(("session", session_token))
        .path("/")
        .max_age(time::Duration::days(5))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let updated_jar = jar.add(cookie);

    Ok((updated_jar, Redirect::to("/dashboard")))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    charac::init().await;
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/callback", get(oauth)) // TODO: make an oauth service for the hca auth instead of leptos
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
