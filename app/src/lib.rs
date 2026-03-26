use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

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
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let get_user_data = Action::new(|_: &()| async move { test().await });
    let msg = get_user_data.value();
    view! {
        <h1 class="text-2xl font-bold text-blue-600 my-8">"CHARA !"</h1>
        <button
            class="text-center text-red-200 mt-30"
            on:click=move |_| { get_user_data.dispatch(()); }
        >
            {move || if get_user_data.pending().get() { "Loading..." } else { "Get your data!" }}
        </button>
        <p class="font-bold text-green-600 mt-15">{move || format!("{:?}", msg.get())}</p>
    }
}
