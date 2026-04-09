use crate::components::ui::{
    button::Button,
    dialog::{
        Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter,
        DialogHeader, DialogTitle, DialogTrigger,
    },
    input::Input,
    label::Label,
};
use leptos::prelude::*;

use super::server::UserBase;

#[component]
pub fn CreateBaseDialog(
    title: impl IntoView + 'static,
    create_action: Action<String, Result<UserBase, ServerFnError>>,
) -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (create_message, set_create_message) = signal::<Option<Result<(), String>>>(None);

    Effect::new(move |_| {
        if let Some(res) = create_action.value().get() {
            let status = match res {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            };

            set_create_message.set(Some(status));

            set_timeout(
                move || {
                    set_create_message.set(None);
                },
                std::time::Duration::from_secs(3),
            );
        }
    });

    //let create_message = move || {
    //    create_action.value().get().map(|res| {
    //        if let Err(e) = res {
    //            view! { <p class="text-destructive text-sm font-medium">{e.to_string()}</p> }
    //                .into_any()
    //        } else {
    //            view! { <p class="text-sm text-muted-foreground">"Base created successfully!"</p> }
    //                .into_any()
    //        }
    //    })
    //};

    view! {
        <Dialog>
            <DialogTrigger>{title}</DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Create a Base!"</DialogTitle>
                        <DialogDescription>
                            "To create a base, you first need a nice name, what could it be :3 ?"
                        </DialogDescription>
                    </DialogHeader>

                    <div class="flex flex-col gap-4 justify-center">
                        <div class="flex flex-col gap-2">
                            <Label html_for="name-1">Name</Label>
                            <Input
                                on:input=move |ev| {
                                    set_name.set(event_target_value(&ev));
                                }
                                prop:value=move || name.get()
                            />
                        </div>
                    </div>

                    <DialogFooter>
                        <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                        <Button
                            attr:r#type="button"
                            attr:disabled=move || create_action.pending().get()
                            on:click=move |_| {
                                create_action.dispatch(name.get());
                                set_name.set("".to_string());
                            }
                        >
                            "Create"
                        </Button>
                    </DialogFooter>
                    {move || {
                        create_message
                            .get()
                            .map(|msg| {
                                match msg {
                                    Err(e) => {
                                        view! {
                                            <p class="text-destructive text-sm font-medium">{e}</p>
                                        }
                                            .into_any()
                                    }
                                    Ok(_) => {
                                        view! {
                                            <p class="text-sm text-muted-foreground">
                                                "Base created successfully!"
                                            </p>
                                        }
                                            .into_any()
                                    }
                                }
                            })
                    }}
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}
