use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::ui::button::Button;
use crate::components::ui::theme_toggle::ThemeToggle;
use leptos::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct UserBase {
    name: String,
    owner_name: String,
    id: String,
}

#[server]
pub async fn get_user_bases() -> Result<Vec<UserBase>, ServerFnError> {
    use std::time::Instant;
    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;

    let bases = service
        .list_bases()
        .await
        .map_err(|e| ServerFnError::new(format!("Listing Bases failed: {e:?}")))?;

    let user_bases = bases
        .into_iter()
        .map(|b| UserBase {
            name: b.name,
            owner_name: format!("{:?}", b.owner.0.key),
            id: b.id.map(|id| format!("{:?}", id.0.key)).unwrap_or_default(),
        })
        .collect();
    let duration = start.elapsed().as_millis();
    println!("[get_user_bases] finished in {}ms", duration);
    Ok(user_bases)
}

#[server]
pub async fn create_base(name: String) -> Result<UserBase, ServerFnError> {
    use std::time::Instant;
    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let base = service
        .create_base(name)
        .await
        .map_err(|e| ServerFnError::new(format!("Base creation failed : {e}")))?;
    let duration = start.elapsed().as_millis();
    println!("[create_base] finished in {}ms", duration);
    Ok(UserBase {
        name: base.name,
        owner_name: format!("{:?}", base.owner.0.key),
        id: base
            .id
            .map(|id| format!("{:?}", id.0.key))
            .unwrap_or_default(),
    })
}
#[component]
pub fn DashboardPage() -> impl IntoView {
    let theme = ThemeMode::init();

    let (refresh_count, set_refresh_count) = signal(0);
    let bases = Resource::new(
        move || refresh_count.get(),
        |_| async move { get_user_bases().await.unwrap() },
    );

    let create_base_action = Action::new(|name: &String| {
        let name = name.clone();
        async move { create_base(name).await.unwrap() }
    });

    Effect::new(move |_| {
        if create_base_action.value().get().is_some() {
            set_refresh_count.update(|n| *n += 1);
        }
    });

    view! {
        <div
            class="relative min-h-screen bg-[var(--background)] text-[var(--foreground)] p-8"
            class:dark=move || theme.is_dark()
        >
            <div class="absolute top-4 right-4 p-2">
                <ThemeToggle />
            </div>

            <div class="flex flex-col gap-6 max-w-4xl mx-auto">
                <h1 class="text-3xl font-bold">"Dashboard"</h1>

                <div class="flex gap-4">
                    <Button on:click=move |_| {
                        if let Some(name) = window().confirm_prompt("Enter base name:") {
                            create_base_action.dispatch(name);
                        }
                    }>
                        {move || if create_base_action.pending().get() { "Creating..." } else { "Create New Base" }}
                    </Button>

                    <Button on:click=move |_| set_refresh_count.update(|n| *n += 1)>
                        "Refresh List"
                    </Button>
                </div>

                <hr class="border-[var(--border)]" />

                <Suspense fallback=move || view! { <p>"Loading bases..."</p> }>
                    {move || Suspend::new(async move {
                        match bases.get() {
                            Some(list) if list.is_empty() => view! { <p>"No bases found."</p> }.into_any(),
                            Some(list) => view! {
                                <ul class="space-y-2">
                                    {list.into_iter().map(|base| view! {
                                        <li class="p-4 border rounded-lg bg-card">
                                            <span class="font-bold">{base.name}</span>
                                            <span class="text-sm opacity-70 ml-2">"(" {base.id} ")"</span>
                                            <p class="text-xs text-muted-foreground">"Owner: " {base.owner_name}</p>
                                        </li>
                                    }).collect_view()}
                                </ul>
                            }.into_any(),
                            _ => view! { <p class="text-red-500">"Unknown error"</p>}.into_any()
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

// Small helper trait to make window prompt cleaner in Rust
trait WindowExt {
    fn confirm_prompt(&self, message: &str) -> Option<String>;
}

impl WindowExt for web_sys::Window {
    fn confirm_prompt(&self, message: &str) -> Option<String> {
        self.prompt_with_message(message).ok().flatten()
    }
}
