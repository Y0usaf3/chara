use super::ui::dropdown_menu::*;
use crate::components::ui::separator::*;
use icons::{Annoyed, Calendar, Clock, Hash, Mail, Shield, User as UserIcon};
use leptos::prelude::*;

#[server]
pub async fn get_user() -> Result<charac::models::User, ServerFnError> {
    let service = crate::get_authenticated_service().await?;
    Ok(service.user)
}

#[component]
pub fn SideNav() -> impl IntoView {
    let user_resource = Resource::new(|| (), |_| async move { get_user().await });

    view! {
        <aside class="fixed bottom-0 left-0 right-0 z-50 w-full h-16 border-t bg-background p-2 flex items-center justify-between px-6">
            <div class="flex items-center gap-2">
                <Annoyed class="size-10 text-[var(--foreground)] border p-1 rounded" />
                <span class="hidden sm:block font-bold text-sm">"CHARA"</span>
            </div>

            <DropdownMenu align=DropdownMenuAlign::EndOuter>
                <DropdownMenuTrigger class="p-0 bg-transparent border-0 outline-none">
                    <div class="flex gap-2 items-center">
                        <Suspense fallback=move || view! {
                            <div class="size-8 rounded-lg bg-secondary animate-pulse" />
                        }>
                            <span class="flex overflow-hidden relative rounded-lg size-8 shrink-0 bg-secondary justify-center items-center font-bold text-xs">
                                {move || user_resource.get().and_then(|res| res.ok()).map(|u| {
                                    u.first_name.chars().next().unwrap_or('?')
                                })}
                            </span>
                        </Suspense>
                    </div>
                </DropdownMenuTrigger>

                <DropdownMenuContent class="w-[260px] mb-2">
                    <Suspense fallback=move || view! { <DropdownMenuLabel>"Loading User..."</DropdownMenuLabel> }>
                        {move || user_resource.get().map(|res| {
                            match res {
                                Ok(user) => view! {
                                    <DropdownMenuLabel>"User Profile"</DropdownMenuLabel>
                                    <DropdownMenuItem class="flex gap-2">
                                        <UserIcon class="size-4 opacity-70" />
                                        <span>{user.first_name} " " {user.last_name}</span>
                                    </DropdownMenuItem>
                                    <DropdownMenuItem class="flex gap-2">
                                        <Mail class="size-4 opacity-70" />
                                        <span>{user.email}</span>
                                    </DropdownMenuItem>
                                    <DropdownMenuItem class="flex gap-2">
                                        <Shield class="size-4 opacity-70" />
                                        <span class="capitalize">{user.role}</span>
                                    </DropdownMenuItem>

                                    <Separator class="my-1" />

                                    <DropdownMenuLabel>"System Data"</DropdownMenuLabel>
                                    <DropdownMenuItem class="flex gap-2">
                                        <Hash class="size-4 opacity-70" />
                                        <span class="text-xs font-mono">
                                            {user.id.map(|id| format!("{:?}", id)).unwrap_or_else(|| "No ID".into())}
                                        </span>
                                    </DropdownMenuItem>

                                    <DropdownMenuItem class="flex gap-2">
                                        <Calendar class="size-4 opacity-70" />
                                        <span class="text-xs text-muted-foreground">
                                            "Created: " {user.created_at.map(|d| d.to_string()).unwrap_or_else(|| "N/A".into())}
                                        </span>
                                    </DropdownMenuItem>

                                    <DropdownMenuItem class="flex gap-2">
                                        <Clock class="size-4 opacity-70" />
                                        <span class="text-xs text-muted-foreground">
                                            "Updated: " {user.updated_at.map(|d| d.to_string()).unwrap_or_else(|| "N/A".into())}
                                        </span>
                                    </DropdownMenuItem>
                                }.into_any(),

                                Err(_) => view! {
                                    <DropdownMenuLabel class="text-destructive">"Failed to load user"</DropdownMenuLabel>
                                }.into_any()
                            }
                        })}
                    </Suspense>

                    <Separator class="my-1" />

                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            <DropdownMenuAction>"Sign Out"</DropdownMenuAction>
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
        </aside>
    }
}
