use crate::components::{
    hooks::use_theme_mode::ThemeMode,
    sidenav::SideNav,
    ui::{
        breadcrumb::{
            Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator,
        },
        button::{Button, ButtonSize, ButtonVariant},
        context_menu::{
            ContextMenu, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuTrigger,
        },
        data_table::{
            DataTable, DataTableBody, DataTableCell, DataTableHead, DataTableHeader, DataTableRow,
            DataTableWrapper,
        },
        empty::*,
        theme_toggle::ThemeToggle,
    },
};
use charac::models::field::FieldConfig;
use components::{CellInput, CreateFieldDialog, DeleteRecordDialog, EditFieldDialog};
use icons::{FolderCode, Plus, Trash2};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use server::{
    add_field, add_record, delete_field, delete_record, get_table_data, update_cell, update_field,
};

mod components;
pub mod server;

use crate::components::ui::sonner::{
    SonnerContainer, SonnerList, SonnerPosition, SonnerToaster, ToastType, provide_toast_context,
    use_toast,
};

#[component]
pub fn TableViewPage() -> impl IntoView {
    let theme = ThemeMode::init();
    let params = use_params_map();
    provide_toast_context();
    let toast_manager = use_toast();
    let table_id = move || params.with(|p| p.get("id").unwrap_or_default());

    let (refresh_count, set_refresh_count) = signal(0);
    let data = Resource::new(
        move || (table_id(), refresh_count.get()),
        |(id, _)| async move { get_table_data(id).await },
    );

    let add_record_action = Action::new(move |id: &String| {
        let id = id.clone();
        async move { add_record(id).await }
    });

    let add_field_action = Action::new(move |(name, field_type): &(String, String)| {
        let name = name.clone();
        let field_type = field_type.clone();
        let id = table_id();
        async move { add_field(id, name, field_type).await }
    });

    let update_field_action = Action::new(
        move |(_table_id, field_id, name, config): &(String, String, String, FieldConfig)| {
            let table_id = table_id();
            let field_id = field_id.clone();
            let name = name.clone();
            let config = config.clone();
            async move { update_field(table_id, field_id, name, config).await }
        },
    );

    let delete_record_action = Action::new(move |(table_id, record_id): &(String, String)| {
        let table_id = table_id.clone();
        let record_id = record_id.clone();
        async move { delete_record(table_id, record_id).await }
    });

    let delete_field_action = Action::new(move |(table_id, field_id): &(String, String)| {
        let table_id = table_id.clone();
        let field_id = field_id.clone();
        async move { delete_field(table_id, field_id).await }
    });

    let update_cell_action = Action::new(
        move |(record_id, field_id, value): &(String, String, String)| {
            let table_id = table_id();
            let record_id = record_id.clone();
            let field_id = field_id.clone();
            let value = value.clone();
            async move { update_cell(table_id, record_id, field_id, value).await }
        },
    );

    Effect::new(move |_| {
        let _ = add_record_action.version().get();
        let _ = add_field_action.version().get();
        let _ = delete_record_action.version().get();
        let _ = delete_field_action.version().get();
        let _ = update_field_action.version().get();

        if add_record_action.version().get() > 0
            || add_field_action.version().get() > 0
            || delete_record_action.version().get() > 0
            || delete_field_action.version().get() > 0
            || update_field_action.version().get() > 0
        {
            set_refresh_count.update(|n| *n += 1);
        }
    });
    Effect::new(move |_| {
        if let Some(result) = update_cell_action.value().get()
            && let Err(e) = result
        {
            toast_manager.push("Error!", "SMT WRONG HAPPENED RAH", ToastType::Error);
        }
    });

    view! {
        <SonnerToaster />
        <div
            class:dark=move || theme.is_dark()
            class="flex min-h-screen bg-[var(--background)] text-[var(--foreground)]"
        >
            <SideNav />
            <div class="flex-1 relative flex flex-col h-screen overflow-hidden">
                <div class="p-8 pb-0 flex flex-col gap-6 w-full">
                    <div class="flex justify-between items-center">
                        <Breadcrumb>
                            <BreadcrumbList>
                                <BreadcrumbItem>
                                    <BreadcrumbLink attr:href="/dashboard">
                                        "Dashboard"
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbLink attr:href="#">"Table"</BreadcrumbLink>
                                </BreadcrumbItem>
                            </BreadcrumbList>
                        </Breadcrumb>

                        <div class="flex gap-2 items-center">
                            <ThemeToggle />
                        </div>
                    </div>

                    <Suspense fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        {move || Suspend::new(async move {
                            match data.await {
                                Ok(table_data) => {
                                    let table_data_name = table_data.name.clone();
                                    let fields_header = table_data.fields.clone();
                                    let fields_body = table_data.fields.clone();
                                    let records = table_data.records.clone();

                                    view! {
                                        <div class="flex justify-between items-center mb-2">
                                            <h1 class="text-2xl font-bold">{table_data_name}</h1>
                                            <div class="flex gap-2">
                                                <CreateFieldDialog add_field_action=add_field_action />
                                                <Button
                                                    variant=ButtonVariant::Outline
                                                    size=ButtonSize::Sm
                                                    on:click=move |_| {
                                                        add_record_action.dispatch(table_id());
                                                    }
                                                >
                                                    <Plus class="mr-1" />
                                                    "Record"
                                                </Button>
                                            </div>
                                        </div>

                                        <div class="flex-1 overflow-auto border rounded-md bg-card">
                                            {if fields_header.is_empty() {
                                                view! {
                                                    <Empty class="h-full flex items-center justify-center p-12">
                                                        <EmptyHeader>
                                                            <EmptyMedia variant=EmptyMediaVariant::Icon>
                                                                <FolderCode />
                                                            </EmptyMedia>
                                                            <EmptyTitle>"No Fields Yet"</EmptyTitle>
                                                            <EmptyDescription>
                                                                "Add a field to get started!"
                                                            </EmptyDescription>
                                                        </EmptyHeader>
                                                        <EmptyContent>
                                                            <CreateFieldDialog add_field_action=add_field_action />
                                                        </EmptyContent>
                                                    </Empty>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <DataTableWrapper class="border-none rounded-none">
                                                        <DataTable class="w-max min-w-full table-fixed">
                                                            <DataTableHeader>
                                                                <DataTableRow>
                                                                    {fields_header
                                                                        .into_iter()
                                                                        .map(|f| {
                                                                            let field = f.clone();
                                                                            let fid = f.id.clone();
                                                                            let fname = f.name.clone();

                                                                            view! {
                                                                                <DataTableHead class="px-4 font-bold group w-[200px] border-r">
                                                                                    <div class="flex justify-between items-center gap-2">
                                                                                        <span class="truncate">{fname.clone()}</span>
                                                                                        <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                                                                                            <EditFieldDialog
                                                                                                field_id=fid.clone()
                                                                                                initial_name=fname
                                                                                                initial_config=field.config.clone()
                                                                                                update_field_action=update_field_action
                                                                                            />
                                                                                            <button
                                                                                                class="p-1 hover:bg-destructive/10 hover:text-destructive rounded transition-colors"
                                                                                                on:click=move |_| {
                                                                                                    delete_field_action.dispatch((table_id(), fid.clone()));
                                                                                                }
                                                                                            >
                                                                                                <Trash2 class="size-3" />
                                                                                            </button>
                                                                                        </div>
                                                                                    </div>
                                                                                </DataTableHead>
                                                                            }
                                                                                .into_any()
                                                                        })
                                                                        .collect_view()}
                                                                </DataTableRow>
                                                            </DataTableHeader>
                                                            <DataTableBody>
                                                                {records
                                                                    .into_iter()
                                                                    .map(|record| {
                                                                        let rid = record.id.clone();
                                                                        let rrid = record.id.clone();
                                                                        let fields = fields_body.clone();
                                                                        let cells = record.cells.clone();
                                                                        view! {
                                                                            <ContextMenu>
                                                                                <ContextMenuTrigger>
                                                                                    <DataTableRow>
                                                                                        {fields
                                                                                            .into_iter()
                                                                                            .map(|field| {
                                                                                                let rid = rid.clone();
                                                                                                let fid = field.id.clone();
                                                                                                let fname = field.name.clone();
                                                                                                let val = cells.get(&fname).cloned().unwrap_or_default();
                                                                                                let config = field.config.clone();

                                                                                                view! {
                                                                                                    <DataTableCell class="px-2 p-1 border-r overflow-hidden">
                                                                                                        <CellInput
                                                                                                            initial_value=val
                                                                                                            config=config
                                                                                                            on_change=Callback::new(move |new_val| {
                                                                                                                update_cell_action
                                                                                                                    .dispatch((rid.clone(), fid.clone(), new_val));
                                                                                                            })
                                                                                                        />
                                                                                                    </DataTableCell>
                                                                                                }
                                                                                            })
                                                                                            .collect_view()}
                                                                                    </DataTableRow>
                                                                                </ContextMenuTrigger>
                                                                                <ContextMenuContent>
                                                                                    <ContextMenuGroup>
                                                                                        <ContextMenuItem>
                                                                                            <DeleteRecordDialog
                                                                                                table_id=table_id()
                                                                                                record_id=rrid
                                                                                                delete_action=delete_record_action
                                                                                            />
                                                                                        </ContextMenuItem>
                                                                                    </ContextMenuGroup>
                                                                                </ContextMenuContent>
                                                                            </ContextMenu>
                                                                        }
                                                                    })
                                                                    .collect_view()}
                                                                <DataTableRow
                                                                    class="hover:bg-muted/30 cursor-pointer group"
                                                                    on:click=move |_| {
                                                                        add_record_action.dispatch(table_id());
                                                                    }
                                                                >
                                                                    <DataTableCell
                                                                        attr:colspan=fields_body.len()
                                                                        class="px-4 py-2 text-muted-foreground/50 group-hover:text-muted-foreground transition-colors"
                                                                    >
                                                                        <div class="flex items-center gap-2 pointer-events-none">
                                                                            <Plus class="size-4" />
                                                                            "Add Record"
                                                                        </div>
                                                                    </DataTableCell>
                                                                </DataTableRow>
                                                            </DataTableBody>
                                                        </DataTable>
                                                    </DataTableWrapper>
                                                }
                                                    .into_any()
                                            }}
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! { <p class="text-red-500">{format!("Error: {}", e)}</p> }
                                        .into_any()
                                }
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
