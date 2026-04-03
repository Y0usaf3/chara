use axum::extract::ConnectInfo;
use axum::extract::Query;
use axum::{http::StatusCode, response::Redirect};
use axum_extra::TypedHeader;
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::SameSite;
use charac::HCAUTH;
use charac::service::user::UserService;
use headers::UserAgent;
use serde::Deserialize;
use std::net::SocketAddr;

// NOTE: NEVER, NEVER, use axum extra again, NEVER!
// NOTE: oh actually u can use it, but ONLY IN THE SERVER WORKSPACE!

#[derive(Deserialize)]
pub struct Code {
    code: String,
}

pub async fn oauth(
    Query(params): Query<Code>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: PrivateCookieJar,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> Result<(PrivateCookieJar, Redirect), StatusCode> {
    let user_agent = user_agent
        .map(|ua| ua.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let ip = addr.ip().to_string();

    let service = UserService::register(params.code).await.map_err(|e| {
        eprintln!("Registration error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let tmp = service.create_session(ip, user_agent).await;
    let session_token = tmp.ok().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = Cookie::build(("session", session_token))
        .path("/")
        .max_age(time::Duration::days(5))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let updated_jar = jar.add(cookie);

    Ok((updated_jar, Redirect::to("/dashboard")))
}

// NOTE: for now it returns a Result<R, S> so if we ever wanted to add some kind of blocker if we
// wanted to block certain ips and stuff like that (i literally did that bc Redirect wont work as a
// return type)
#[axum::debug_handler]
pub async fn redirect_to_oauth() -> Result<Redirect, StatusCode> {
    Ok(Redirect::to(
        &HCAUTH.get_oauth_uri(&["openid", "profile", "email", "name"]),
    ))
}
