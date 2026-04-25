use crate::components::{
    hooks::use_theme_mode::ThemeMode,
    sidenav::SideNav,
    ui::{
        breadcrumb::{
            Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator,
        },
        empty::*,
        theme_toggle::ThemeToggle,
    },
};
use components::{CreateTableDialog, TableBox};
use icons::{ArrowUpRight, FolderCode, Lock, Plus};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use server::{create_table_in_base, get_base_tables};

mod components;
pub mod server;

#[component]
pub fn BasePage() -> impl IntoView {
    let theme = ThemeMode::init();
    let params = use_params_map();
    let base_id = move || params.with(|p| p.get("id").unwrap_or_default());

    let (refresh_count, set_refresh_count) = signal(0);
    let tables = Resource::new(
        move || (base_id(), refresh_count.get()),
        |(id, _)| async move { get_base_tables(id).await },
    );

    let create_table_action = Action::new(move |name: &String| {
        let name = name.clone();
        let id = base_id();
        async move { create_table_in_base(id, name).await }
    });

    Effect::new(move |_| {
        if create_table_action.value().get().is_some() {
            set_refresh_count.update(|n| *n += 1);
        }
    });

    view! {
        <div
            class:dark=move || theme.is_dark()
            class="flex min-h-screen bg-[var(--background)] text-[var(--foreground)]"
        >
            <SideNav />
            <div class="flex-1 relative p-8">
                <div class="absolute top-4 right-4 p-2">
                    <ThemeToggle />
                </div>

                <div class="flex flex-col gap-6 h-full w-full">
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink attr:href="/dashboard">"Dashboard"</BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbLink attr:href=format!(
                                    "/base/{}",
                                    base_id(),
                                )>"Base"</BreadcrumbLink>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>

                    <div class="flex gap-4 justify-between items-center">
                        <h1 class="text-2xl font-bold">"Tables"</h1>
                        <CreateTableDialog
                            title=move || {
                                if create_table_action.pending().get() {
                                    view! { <Lock /> }.into_any()
                                } else {
                                    view! { <Plus /> }.into_any()
                                }
                            }
                            create_action=create_table_action
                        />
                    </div>

                    <Suspense>
                        {move || Suspend::new(async move {
                            match tables.get() {
                                Some(Ok(list)) if list.is_empty() => {
                                    view! {
                                        <Empty class="flex-1 flex items-center justify-center">
                                            <EmptyHeader>
                                                <EmptyMedia variant=EmptyMediaVariant::Icon>
                                                    <FolderCode />
                                                </EmptyMedia>
                                                <EmptyTitle>"No Table Yet"</EmptyTitle>
                                                <EmptyDescription>
                                                    "This base is empty. Create your first table to start organizing data! :3"
                                                </EmptyDescription>
                                            </EmptyHeader>

                                            <EmptyContent>
                                                <div class="flex gap-2">
                                                    <CreateTableDialog
                                                        title="Create Table".into_any()
                                                        create_action=create_table_action
                                                    />
                                                </div>
                                            </EmptyContent>
                                        </Empty>
                                    }
                                        .into_any()
                                }
                                Some(Ok(list)) => {
                                    view! {
                                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                            {list
                                                .into_iter()
                                                .map(|table| {
                                                    view! { <TableBox table=table /> }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                                Some(Err(e)) => {
                                    view! {
                                        <p class="text-red-500">
                                            {format!("Error loading tables: {}", e)}
                                        </p>
                                    }
                                        .into_any()
                                }
                                _ => view! { <p>"Loading..."</p> }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
