use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[server]
pub async fn get_user_dashboard() -> Result<String, ServerFnError> {
    use axum::extract::ConnectInfo;
    use axum::http::HeaderMap;
    use axum_extra::extract::cookie::CookieJar;
    use charac::service::user::*;
    use leptos_axum::extract;
    use std::net::SocketAddr;

    let jar: CookieJar = extract().await?;
    let ConnectInfo(addr): ConnectInfo<SocketAddr> = extract().await?;
    let headers: HeaderMap = extract().await?;

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = jar
        .get("session")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ServerFnError::new("No token cookie. Please log in."))?;

    let service = UserService::login(AuthMethod::Session(SessionI {
        token,
        ip: addr.ip().to_string(),
        agent: user_agent.to_string(),
    }))
    .await
    .ok()
    .ok_or_else(|| ServerFnError::new("Invalid session or expired token."))?;

    Ok(format!("Welcome back! {:?}", service.user))
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let user_data = RwSignal::new("loading...".to_string());

    Effect::new(move |_| {
        spawn_local(async move {
            let res = get_user_dashboard().await;
            user_data.set(format!("{:#?}", res));
        });
    });

    view! {
        <div class="min-h-screen bg-slate-50 flex items-center justify-center p-4">
            <div class="w-full max-w-md bg-white rounded-2xl shadow-xl border border-slate-100 p-8">

                <div class="flex flex-col items-center space-y-6">
                    <div class="text-center space-y-2">
                        <h2 class="text-2xl font-bold text-slate-800">"Dashboard"</h2>
                        <p class="text-sm text-slate-500">"server"</p>
                    </div>

                    <div class="w-full bg-slate-50 rounded-lg p-4 border border-slate-200">
                        <p class="text-slate-700 font-mono text-sm break-all whitespace-pre-wrap">
                            {move || user_data.get()}
                        </p>
                    </div>

                </div>

            </div>
        </div>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[server]
pub async fn test() -> Result<String, ServerFnError> {
    use axum::extract::ConnectInfo;
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use std::net::SocketAddr;
    let headers: HeaderMap = extract().await?;
    let ConnectInfo(addr): ConnectInfo<SocketAddr> = extract().await?;

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    Ok(format!(
        "it smh worked ??? here is the user sir! IP: {} USER_AGENT: {}",
        addr.ip(),
        user_agent
    ))
}

#[server]
pub async fn get_oauth_link() -> Result<String, ServerFnError> {
    Ok(charac::HCAUTH.get_oauth_uri(&["openid", "profile", "email", "name"]))
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/chara.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("dashboard") view=Dashboard />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-bold text-blue-600 my-8">"CHARA !"</h1>
        <button
            class="text-center text-red-200 mt-30"
            on:click=move |_| spawn_local(async move {
                let url = get_oauth_link().await.unwrap_or("https://example.com".to_string());
                let _ = window().location().set_href(&url);
            })
        >
            "Oauth with Hackclub Auth!"
        </button>
    }
}
