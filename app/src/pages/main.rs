use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::ui::button::Button;
use crate::components::ui::theme_toggle::ThemeToggle;

use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
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
                <img class="w-120 h-auto rounded-sm border" src="chaira.png" />

                <div class="flex gap-4">
                    <Button on:click=move |_| {
                        let _ = window().location().assign("/auth");
                    }>"OAuth with Hackclub Auth!"</Button>
                </div>
            </div>
        </div>
    }
}
