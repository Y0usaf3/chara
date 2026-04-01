use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::ui::theme_toggle::ThemeToggle;
use charac::service::user::UserService;
use leptos::prelude::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserBase {
    name: String,
    owner_name: String,
    id: String,
}

#[server]
pub async fn get_user_bases() -> Result<Vec<UserBase>, ServerFnError> {
    use axum_extra::extract::cookie::PrivateCookieJar;
    use leptos_axum::extract_with_state;
    let state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("AppState not found in context"))?;

    let jar = extract_with_state::<PrivateCookieJar, crate::AppState>(&state)
        .await
        .map_err(|e| ServerFnError::new(format!("Cookie extraction failed: {e:?}")))?;

    let secret = jar
        .get("session")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ServerFnError::new("No session cookie found"))?;
    let service = UserService::login(charac::service::user::AuthMethod::Session(secret))
        .await
        .map_err(|_| ServerFnError::ServerError("aaaaaaa".to_string()))?;
    let bases = service
        .list_bases()
        .await
        .map_err(|_| ServerFnError::ServerError("bbbbbbbbbbbb".to_string()))?;
    let user_bases = bases
        .into_iter()
        .map(|b| UserBase {
            name: b.name,
            owner_name: b.owner,
            id: b.id.map(|id| id.to_string()).unwrap_or_default(),
        })
        .collect();
    Ok(user_bases)
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let theme = ThemeMode::init();

    view! {
        <div
            class="relative min-h-screen bg-[var(--background)] text-[var(--foreground)]"
            class:dark=move || theme.is_dark()
        >

            <div class="absolute top-4 right-4 p-2">
                <ThemeToggle />
            </div>

        </div>
    }
}
