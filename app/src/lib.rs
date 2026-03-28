mod components;
use components::ui::button::{Button, ButtonVariant};
use components::ui::theme_toggle::ThemeToggle;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::components::hooks::use_theme_mode::ThemeMode;

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
    let theme = ThemeMode::init();

    view! {
        <div
            class="relative min-h-screen bg-[var(--background)] text-[var(--foreground)]"
            class:dark=move || theme.is_dark()
        >

            <div class="absolute top-4 right-4 p-2">
                <ThemeToggle />
            </div>

            <div class="flex flex-col items-center justify-center min-h-screen gap-6">
                <img
                    class="w-40 h-auto"
                    src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%2Fid%2FOIP.SpWSlKomy6sWbIwXJ8dsNgHaM-%3Fpid%3DApi&f=1&ipt=4a32cad500d8c6b9988ef34a78e8fe8eba544f6a176e40c50a87083c4ce65a0c"
                />

                <div class="flex gap-4">
                    <Button href="/auth">"OAuth with Hackclub Auth!"</Button>
                </div>
            </div>

        </div>
    }
}
