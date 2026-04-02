use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub async fn get_authenticated_service() -> Result<charac::service::user::UserService, ServerFnError>
{
    use axum::extract::ConnectInfo;
    use axum::http::HeaderMap;
    use axum_extra::extract::cookie::PrivateCookieJar;
    use charac::service::user::{AuthMethod, Session, UserService};
    use leptos::context::use_context;
    use leptos::prelude::ServerFnError;
    use leptos_axum::{extract, extract_with_state};
    use std::net::SocketAddr;
    let ConnectInfo(addr): ConnectInfo<SocketAddr> = extract()
        .await
        .map_err(|e| ServerFnError::new(format!("Connection info missing: {e:?}")))?;

    let headers: HeaderMap = extract()
        .await
        .map_err(|e| ServerFnError::new(format!("Headers missing: {e:?}")))?;

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("AppState not found in context"))?;

    let jar = extract_with_state::<PrivateCookieJar, crate::AppState>(&state)
        .await
        .map_err(|e| ServerFnError::new(format!("Cookie extraction failed: {e:?}")))?;

    let secret = jar
        .get("session")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ServerFnError::new("No session cookie found"))?;

    let service = UserService::login(AuthMethod::Session(Session {
        token: secret,
        ip: addr.ip().to_string(),
        agent: user_agent.to_string(),
    }))
    .await
    .map_err(|e| ServerFnError::new(format!("Authentication failed: {e:?}")))?;

    Ok(service)
}
