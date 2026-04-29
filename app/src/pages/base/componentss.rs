use crate::components::ui::{
    button::Button,
    dialog::{
        Dialog, DialogBody, DialogClose, DialogContent, DialogCustomTrigger, DialogDescription,
        DialogFooter, DialogHeader, DialogTitle,
    },
    input::Input,
    label::Label,
};
use leptos::prelude::*;
use std::rc::Rc;

use super::server::BaseTable;

#[component]
pub fn CreateTableDialog(
    title: impl IntoView + 'static,
    create_action: Action<String, Result<BaseTable, ServerFnError>>,
) -> impl IntoView {
    let name = ArcRwSignal::new("".to_string());
    let name_effect = name.clone();
    let name_input = name.clone();
    let name_keydown = name.clone();
    let name_click = name.clone();

    Effect::new(move |_| {
        if create_action.value().get().is_some() {
            name_effect.set("".to_string());
        }
    });

    view! {
        <Dialog>
            <DialogCustomTrigger>{title}</DialogCustomTrigger>
            <DialogContent class="sm:max-w-[500px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Create a Table!"</DialogTitle>
                        <DialogDescription>"Give your table a name."</DialogDescription>
                    </DialogHeader>
                    <div class="flex flex-col gap-4 py-4">
                        <Input
                            bind_value=name_input
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    let val = name_keydown.get_untracked();
                                    if !val.is_empty() {
                                        leptos::task::spawn_local(async move {
                                            create_action.dispatch(val);
                                        });
                                        name_keydown.set("".to_string());
                                    }
                                }
                            }
                        />
                    </div>
                    <DialogFooter>
                        <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                        <Button attr:r#type="button" on:click=move |_| {
                            let val = name_click.get_untracked();
                            if !val.is_empty() {
                                leptos::task::spawn_local(async move {
                                    create_action.dispatch(val);
                                });
                                name_click.set("".to_string());
                            }
                        }>
                            "Create"
                        </Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn CreateFieldDialog(
    title: impl IntoView + 'static,
    create_action: Action<String, Result<super::server::TableField, ServerFnError>>,
) -> impl IntoView {
    let name = ArcRwSignal::new("".to_string());
    let name_input = name.clone();
    let name_keydown = name.clone();
    let name_click = name.clone();

    view! {
        <Dialog>
            <DialogCustomTrigger>{title}</DialogCustomTrigger>
            <DialogContent class="min-w-[300px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Add a Field"</DialogTitle>
                    </DialogHeader>
                    <div class="flex flex-col gap-4 py-4">
                        <Input
                            id="field-name"
                            bind_value=name_input
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    let val = name_keydown.get_untracked();
                                    if !val.is_empty() {
                                        create_action.dispatch(val);
                                        name_keydown.set("".to_string());
                                    }
                                }
                            }
                        />
                    </div>
                    <DialogFooter>
                        <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                        <Button
                            attr:r#type="button"
                            on:click=move |_| {
                                let val = name_click.get_untracked();
                                if !val.is_empty() {
                                    create_action.dispatch(val);
                                    name_click.set("".to_string());
                                }
                            }
                        >
                            "Add Field"
                        </Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn RenameFieldDialog(
    title: impl IntoView + 'static,
    current_name: String,
    rename_action: Action<(String, String), Result<(), ServerFnError>>,
    field_id: String,
) -> impl IntoView {
    let field_id_stored = StoredValue::new(field_id);
    let name = ArcRwSignal::new(current_name);
    let name_input = name.clone();
    let name_keydown = name.clone();
    let name_click = name.clone();

    view! {
        <Dialog>
            <DialogCustomTrigger>{title}</DialogCustomTrigger>
            <DialogContent class="min-w-[300px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Rename Field"</DialogTitle>
                        <DialogDescription>"Enter a new name for your field."</DialogDescription>
                    </DialogHeader>

                    <div class="flex flex-col gap-4 py-4">
                        <div class="flex flex-col gap-2">
                            <Label html_for="rename-field-name">Name</Label>
                            <Input
                                id="rename-field-name"
                                bind_value=name_input
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        let current_val = name_keydown.get_untracked();
                                        let id = field_id_stored.get_value();
                                        if !current_val.is_empty() {
                                            leptos::task::spawn_local(async move {
                                                rename_action.dispatch((id, current_val));
                                            });
                                        }
                                    }
                                }
                            />
                        </div>
                    </div>

                    <DialogFooter class="sm:justify-center flex-col sm:flex-row gap-3">
                        <DialogClose class="w-full sm:w-[120px]">"Cancel"</DialogClose>
                        <Button
                            attr:r#type="button"
                            class="w-full sm:w-[120px]"
                            on:click=move |_| {
                                let current_val = name_click.get_untracked();
                                let id = field_id_stored.get_value();
                                if !current_val.is_empty() {
                                    leptos::task::spawn_local(async move {
                                        rename_action.dispatch((id, current_val));
                                    });
                                }
                            }
                        >
                            "Rename"
                        </Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn TableBox(table: BaseTable) -> impl IntoView {
    view! {
        <div
            class="p-4 border rounded-lg bg-card hover:bg-accent cursor-pointer transition-colors"
            on:click=move |_| {
                window().location().assign(format!("/base/{}", table.id.clone()).as_str()).unwrap()
            }
        >
            <span class="font-bold text-lg">{table.name}</span>
            <p class="text-xs text-muted-foreground">"ID: " {table.id.clone()}</p>
        </div>
    }
}
