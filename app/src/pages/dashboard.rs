use crate::components::{
    hooks::use_theme_mode::ThemeMode,
    sidenav::SideNav,
    ui::{
        breadcrumb::{
            Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator,
        },
        button::{Button, ButtonSize, ButtonVariant},
        dialog::{
            Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter,
            DialogHeader, DialogTitle, DialogTrigger,
        },
        empty::*,
        input::Input,
        label::Label,
        theme_toggle::ThemeToggle,
    },
};
use icons::{ArrowUpRight, FolderCode, Lock, Plus};
use leptos::prelude::*;

// TODO: make a component for creating a base

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct UserBase {
    name: String,
    owner_name: String,
    id: String,
}

#[component]
pub fn CreateBaseDialog(
    title: impl IntoView + 'static,
    create_action: Action<String, Result<UserBase, ServerFnError>>,
) -> impl IntoView {
    let (name, set_name) = signal("".to_string());

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_action.value().get() {
            set_name.set("".to_string());
        }
    });

    view! {
        <Dialog>
            <DialogTrigger>{title}</DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Create a Base!"</DialogTitle>

                        <DialogDescription>
                            {move || {
                                create_action
                                    .value()
                                    .get()
                                    .and_then(|res| {
                                        if let Err(e) = res {
                                            Some(
                                                view! {
                                                    <p class="text-destructive text-sm font-medium">
                                                        {e.to_string()}
                                                    </p>
                                                }
                                                    .into_any(),
                                            )
                                        } else {
                                            Some(
                                                "To create a base, you first need a nice name, what could it be :3 ?"
                                                    .into_any(),
                                            )
                                        }
                                    })
                            }}
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
                            on:click=move |_| {
                                create_action.dispatch(name.get());
                                set_name.set("".to_string());
                            }
                        >
                            "Create"
                        </Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
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
        .map_err(|e| ServerFnError::new(format!("{e}")))?;
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
        |_| async move { get_user_bases().await },
    );

    let create_base_action = Action::new(|name: &String| {
        let name = name.clone();
        async move { create_base(name).await }
    });

    Effect::new(move |_| {
        if create_base_action.value().get().is_some() {
            set_refresh_count.update(|n| *n += 1);
        }
    });

    let (name, set_name) = signal("".to_string());

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
                        </BreadcrumbList>
                    </Breadcrumb>

                    <div class="flex gap-4 justify-end">
                        // TODO: fix the dialog so it clears the input when we created a base and
                        // returns error when there is one
                        // and also fix the issue where the dialog is not closed when using the +
                        // btn for some reason
                        <CreateBaseDialog
                            title=move || {
                                if create_base_action.pending().get() {
                                    view! { <Lock /> }.into_any()
                                } else {
                                    view! { <Plus /> }.into_any()
                                }
                            }
                            create_action=create_base_action
                        />
                        //<Dialog>
                        //    <DialogTrigger>
                        //        {move || {
                        //            if create_base_action.pending().get() {
                        //                view! { <Lock /> }.into_any()
                        //            } else {
                        //                view! { <Plus /> }.into_any()
                        //            }
                        //        }}
                        //    </DialogTrigger>
                        //
                        //    <DialogContent class="sm:max-w-[425px]">
                        //        <DialogBody>
                        //            <DialogHeader>
                        //                <DialogTitle>"Create a Base!"</DialogTitle>
                        //
                        //                <DialogDescription>
                        //                    {move || {
                        //                        create_base_action
                        //                            .value()
                        //                            .get()
                        //                            .and_then(|res| {
                        //                                if let Err(e) = res {
                        //                                    Some(
                        //                                        view! {
                        //                                            <p class="text-destructive text-sm font-medium">
                        //                                                {e.to_string()}
                        //                                            </p>
                        //                                        }
                        //                                            .into_any(),
                        //                                    )
                        //                                } else {
                        //                                    Some(
                        //                                        "To create a base, you first need a nice name, what could it be :3 ?"
                        //                                            .into_any(),
                        //                                    )
                        //                                }
                        //                            })
                        //                    }}
                        //                </DialogDescription>
                        //            </DialogHeader>
                        //
                        //            <div class="flex flex-col gap-4 justify-center">
                        //                <div class="flex flex-col gap-2">
                        //                    <Label html_for="name-1">Name</Label>
                        //                    <Input
                        //                        on:input=move |ev| {
                        //                            set_name.set(event_target_value(&ev));
                        //                        }
                        //                        prop:value=move || name.get()
                        //                    />
                        //                </div>
                        //            </div>
                        //
                        //            <DialogFooter>
                        //                <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                        //                <Button
                        //                    attr:r#type="button"
                        //                    on:click=move |_| {
                        //                        create_base_action.dispatch(name.get());
                        //                        set_name.set("".to_string());
                        //                    }
                        //                >
                        //                    "Create"
                        //                </Button>
                        //            </DialogFooter>
                        //        </DialogBody>
                        //    </DialogContent>
                        //</Dialog>
                    </div>

                    <Suspense fallback=move || {
                        view! { <p>"Loading bases..."</p> }
                    }>
                        {move || Suspend::new(async move {
                            match bases.get() {
                                Some(Ok(list)) if list.is_empty() => {
                                    view! {
                                        <Empty class="flex-1 flex items-center justify-center">
                                            <EmptyHeader>
                                                <EmptyMedia variant=EmptyMediaVariant::Icon>
                                                    <FolderCode />
                                                </EmptyMedia>
                                                <EmptyTitle>"No Base Yet"</EmptyTitle>
                                                <EmptyDescription>
                                                    {move || {
                                                        create_base_action
                                                            .value()
                                                            .get()
                                                            .and_then(|res| {
                                                                if let Err(e) = res {
                                                                    Some(
                                                                        view! {
                                                                            <p class="text-destructive text-sm font-medium">
                                                                                {e.to_string()}
                                                                            </p>
                                                                        }
                                                                            .into_any(),
                                                                    )
                                                                } else {
                                                                    Some(
                                                                        "You haven't created any bases yet. Get started by creating your first base! :3"
                                                                            .into_any(),
                                                                    )
                                                                }
                                                            })
                                                    }}
                                                </EmptyDescription>
                                            </EmptyHeader>

                                            <EmptyContent>
                                                <div class="flex gap-2">
                                                    <Dialog>
                                                        <DialogTrigger>"Create Base"</DialogTrigger>
                                                        <DialogContent class="sm:max-w-[425px]">
                                                            <DialogBody>
                                                                <DialogHeader>
                                                                    <DialogTitle>"Create a Base!"</DialogTitle>

                                                                    <DialogDescription>
                                                                        {move || {
                                                                            create_base_action
                                                                                .value()
                                                                                .get()
                                                                                .and_then(|res| {
                                                                                    if let Err(e) = res {
                                                                                        Some(
                                                                                            view! {
                                                                                                <p class="text-destructive text-sm font-medium">
                                                                                                    {e.to_string()}
                                                                                                </p>
                                                                                            }
                                                                                                .into_any(),
                                                                                        )
                                                                                    } else {
                                                                                        Some(
                                                                                            "To create a base, you first need a nice name, what could it be :3 ?"
                                                                                                .into_any(),
                                                                                        )
                                                                                    }
                                                                                })
                                                                        }}
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
                                                                        on:click=move |_| {
                                                                            create_base_action.dispatch(name.get());
                                                                            set_name.set("".to_string());
                                                                        }
                                                                    >
                                                                        "Create"
                                                                    </Button>
                                                                </DialogFooter>
                                                            </DialogBody>
                                                        </DialogContent>
                                                    </Dialog>

                                                </div>

                                                <Button
                                                    variant=ButtonVariant::Link
                                                    size=ButtonSize::Sm
                                                    class="text-muted-foreground"
                                                >
                                                    <a href="#" class="flex gap-1 items-center">
                                                        <span>"Learn More"</span>
                                                        <ArrowUpRight />
                                                    </a>
                                                </Button>
                                            </EmptyContent>
                                        </Empty>
                                    }
                                        .into_any()
                                }
                                Some(list) => {

                                    view! {
                                        <div class="grid grid-cols-3 gap-3">
                                            {list
                                                .unwrap()
                                                .into_iter()
                                                .map(|base| {
                                                    view! {
                                                        <div class="p-2 border rounded-lg bg-card">
                                                            <span class="font-bold">{base.name}</span>
                                                            <span class="text-sm opacity-70 ml-2">{base.id}</span>
                                                            <p class="text-xs text-muted-foreground">
                                                                "Owner: " {base.owner_name}
                                                            </p>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                                _ => {
                                    view! { <p class="text-red-500">"Unknown error"</p> }.into_any()
                                }
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
