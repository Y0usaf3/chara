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
                <img
                    class="w-40 h-auto rounded-sm border"
                    src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%2Fid%2FOIP.SpWSlKomy6sWbIwXJ8dsNgHaM-%3Fpid%3DApi&f=1&ipt=4a32cad500d8c6b9988ef34a78e8fe8eba544f6a176e40c50a87083c4ce65a0c"
                />

                <div class="flex gap-4">
                    <Button on:click=move |_| {
                        let _ = window().location().assign("/auth");
                    }>"OAuth with Hackclub Auth!"</Button>
                </div>
            </div>

        </div>
    }
}
