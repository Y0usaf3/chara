use crate::components::{
    hooks::use_theme_mode::ThemeMode,
    sidenav::SideNav,
    ui::{
        breadcrumb::{
            Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator,
        },
        button::{Button, ButtonVariant},
        empty::*,
        theme_toggle::ThemeToggle,
        table::*,
    },
};
use components::CreateTableDialog;
use icons::{
    AlignLeft, Calendar, Cpu, FolderCode, Globe, Hash, Link, List, Lock, Mail, Phone, Plus,
    Settings, Type, User,
};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use server::{create_table_in_base, get_base_tables, get_table_data, TableData};

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

    let (selected_table_id, set_selected_table_id) = signal::<Option<String>>(None);

    let create_table_action = Action::new(move |name: &String| {
        let name = name.clone();
        let id = base_id();
        async move { create_table_in_base(id, name).await }
    });

    Effect::new(move |_| {
        if let Some(Ok(table)) = create_table_action.value().get() {
            set_refresh_count.update(|n| *n += 1);
            set_selected_table_id.set(Some(table.id));
        }
    });

    Effect::new(move |_| {
        if selected_table_id.get().is_none() {
            if let Some(Ok(list)) = tables.get() {
                if !list.is_empty() {
                    set_selected_table_id.set(Some(list[0].id.clone()));
                }
            }
        }
    });

    view! {
        <div
            class:dark=move || theme.is_dark()
            class="flex min-h-screen bg-[var(--background)] text-[var(--foreground)]"
        >
            <SideNav />
            <div class="flex-1 relative flex flex-col min-w-0">
                <div class="absolute top-4 right-4 p-2 z-50">
                    <ThemeToggle />
                </div>

                <div class="flex flex-col gap-4 p-8 pb-0">
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
                        <div class="flex items-center gap-4 overflow-x-auto pb-2 scrollbar-hide">
                            <Suspense>
                                {move || {
                                    tables
                                        .get()
                                        .map(|res| {
                                            match res {
                                                Ok(list) => {
                                                    list.into_iter()
                                                        .map(|table| {
                                                            let id = table.id.clone();
                                                            let is_selected = move || {
                                                                selected_table_id.get() == Some(id.clone())
                                                            };
                                                            view! {
                                                                <button
                                                                    class=move || {
                                                                        format!(
                                                                            "px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap {}",
                                                                            if is_selected() {
                                                                                "border-primary text-primary"
                                                                            } else {
                                                                                "border-transparent text-muted-foreground hover:text-foreground"
                                                                            },
                                                                        )
                                                                    }
                                                                    on:click=move |_| {
                                                                        set_selected_table_id.set(Some(table.id.clone()));
                                                                    }
                                                                >
                                                                    {table.name}
                                                                </button>
                                                            }
                                                                .into_any()
                                                        })
                                                        .collect_view()
                                                        .into_any()
                                                }
                                                _ => view! {}.into_any(),
                                            }
                                        })
                                }}
                            </Suspense>
                            <CreateTableDialog
                                title=move || {
                                    if create_table_action.pending().get() {
                                        view! { <Lock /> }.into_any()
                                    } else {
                                        view! { <Plus class="size-4" /> }.into_any()
                                    }
                                }
                                create_action=create_table_action
                            />
                        </div>
                    </div>
                </div>

                <div class="flex-1 overflow-auto p-8">
                    {move || {
                        if let Some(table_id) = selected_table_id.get() {
                            let b_id = base_id();
                            view! { <TableGrid base_id=b_id table_id=table_id /> }.into_any()
                        } else {
                            view! {
                                <Suspense>
                                    {move || {
                                        tables
                                            .get()
                                            .map(|res| {
                                                if let Ok(list) = res {
                                                    if list.is_empty() {
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
                                                    } else {
                                                        view! {}.into_any()
                                                    }
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            })
                                    }}
                                </Suspense>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn FieldIcon(config: charac::models::field::FieldConfig) -> impl IntoView {
    use charac::models::field::{FieldConfig, TextConfig};
    match config {
        FieldConfig::Text(t) => match t {
            TextConfig::SingleLine { .. } => view! { <Type class="size-4" /> }.into_any(),
            TextConfig::LongText { .. } => view! { <AlignLeft class="size-4" /> }.into_any(),
            TextConfig::Email => view! { <Mail class="size-4" /> }.into_any(),
            TextConfig::URL => view! { <Globe class="size-4" /> }.into_any(),
            TextConfig::Phone => view! { <Phone class="size-4" /> }.into_any(),
        },
        FieldConfig::Number(_) => view! { <Hash class="size-4" /> }.into_any(),
        FieldConfig::Select(_) => view! { <List class="size-4" /> }.into_any(),
        FieldConfig::Datetime(_) => view! { <Calendar class="size-4" /> }.into_any(),
        FieldConfig::Relation(_) => view! { <Link class="size-4" /> }.into_any(),
        FieldConfig::User(_) => view! { <User class="size-4" /> }.into_any(),
        FieldConfig::Computed(_) => view! { <Cpu class="size-4" /> }.into_any(),
        FieldConfig::Custom(_) => view! { <Settings class="size-4" /> }.into_any(),
    }
}

#[component]
fn TableGrid(base_id: String, table_id: String) -> impl IntoView {
    let base_id_sv = StoredValue::new(base_id);
    let table_id_sv = StoredValue::new(table_id);

    let table_data_res = Resource::new(
        move || (base_id_sv.get_value(), table_id_sv.get_value()),
        |(b_id, t_id)| async move { get_table_data(b_id, t_id).await },
    );

    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading table data..."</p> }
        }>
            {move || {
                if let Some(Ok(data)) = table_data_res.get() {
                    let fields_sv = StoredValue::new(data.fields.clone());
                    view! {
                        <TableWrapper class="w-full">
                            <Table class="w-full max-w-none">
                                <TableHeader>
                                    <TableRow>
                                        {fields_sv
                                            .get_value()
                                            .into_iter()
                                            .map(|field| {
                                                view! {
                                                    <TableHead class="font-bold border-r last:border-r-0">
                                                        <div class="flex items-center gap-2">
                                                            <FieldIcon config=field.config.clone() />
                                                            {field.name.clone()}
                                                        </div>
                                                    </TableHead>
                                                }
                                            })
                                            .collect_view()}
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    <For
                                        each=move || data.records.clone()
                                        key=|record| record.id.clone()
                                        let:record
                                    >
                                        {
                                            let record_cells = record.cells.clone();
                                            view! {
                                                <TableRow>
                                                    {fields_sv
                                                        .get_value()
                                                        .into_iter()
                                                        .map({
                                                            let record_cells = record_cells.clone();
                                                            move |field| {
                                                                let field_name = field.name.clone();
                                                                let value = record_cells
                                                                    .get(&field_name)
                                                                    .cloned()
                                                                    .unwrap_or_default();
                                                                view! {
                                                                    <TableCell class="px-3 py-0 h-10 border-r last:border-r-0 overflow-hidden whitespace-nowrap text-ellipsis">
                                                                        <div class="flex items-center h-full">{value}</div>
                                                                    </TableCell>
                                                                }
                                                            }
                                                        })
                                                        .collect_view()}
                                                </TableRow>
                                            }
                                        }
                                    </For>
                                </TableBody>
                            </Table>
                        </TableWrapper>
                    }
                        .into_any()
                } else if let Some(Err(e)) = table_data_res.get() {
                    view! { <p class="text-destructive">{format!("Error: {}", e)}</p> }.into_any()
                } else {
                    view! { <p>"Preparing table..."</p> }.into_any()
                }
            }}
        </Suspense>
    }
}

