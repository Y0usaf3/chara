use crate::components::ui::input::InputType;
use crate::components::ui::{
    button::{Button, ButtonSize, ButtonVariant},
    button_action::ButtonAction,
    dialog::{
        Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter,
        DialogHeader, DialogTitle, DialogTrigger,
    },
    input::Input,
    label::Label,
    select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue},
};
use charac::models::field::FieldConfig;
use icons::{Plus, Settings2, Trash2};
use leptos::prelude::*;

#[component]
pub fn CreateFieldDialog(
    add_field_action: Action<(String, String), Result<(), ServerFnError>>,
) -> impl IntoView {
    let name = RwSignal::new("".to_string());
    let selected_type = RwSignal::new("Text".to_string());

    view! {
        <Dialog>
            <DialogTrigger variant=ButtonVariant::Outline size=ButtonSize::Sm class="gap-1">
                <Plus class="size-4" />
                "Field"
            </DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Create a Field!"</DialogTitle>
                        <DialogDescription>"Add a new field to your table."</DialogDescription>
                    </DialogHeader>

                    <div class="flex flex-col gap-4 py-4">
                        <div class="flex flex-col gap-2">
                            <Label html_for="field-name">"Name"</Label>
                            <Input
                                id="field-name"
                                bind_value=name
                                placeholder="e.g. Price, Description..."
                            />
                        </div>
                        <div class="flex flex-col gap-2">
                            <Label>"Type"</Label>
                            <Select
                                default_value="Text".to_string()
                                on_change=Callback::new(move |val: Option<String>| {
                                    if let Some(v) = val {
                                        selected_type.set(v);
                                    }
                                })
                            >
                                <SelectTrigger>
                                    <SelectValue placeholder="Select type..." />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectOption value="Text".to_string()>"Text"</SelectOption>
                                        <SelectOption value="Long Text"
                                            .to_string()>"Long Text"</SelectOption>
                                        <SelectOption value="Email"
                                            .to_string()>"Email"</SelectOption>
                                        <SelectOption value="URL".to_string()>"URL"</SelectOption>
                                        <SelectOption value="Phone"
                                            .to_string()>"Phone"</SelectOption>
                                        <SelectOption value="Number"
                                            .to_string()>"Number"</SelectOption>
                                        <SelectOption value="Decimal"
                                            .to_string()>"Decimal"</SelectOption>
                                        <SelectOption value="Currency"
                                            .to_string()>"Currency"</SelectOption>
                                        <SelectOption value="Percent"
                                            .to_string()>"Percent"</SelectOption>
                                        <SelectOption value="Rating"
                                            .to_string()>"Rating"</SelectOption>
                                        <SelectOption value="Datetime"
                                            .to_string()>"Datetime"</SelectOption>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                    </div>

                    <DialogFooter>
                        <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                        <Button
                            attr:r#type="button"
                            attr:disabled=move || add_field_action.pending().get()
                            on:click=move |_| {
                                add_field_action.dispatch((name.get(), selected_type.get()));
                                name.set("".to_string());
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

#[component]
pub fn EditFieldDialog(
    field_id: String,
    initial_name: String,
    initial_config: FieldConfig,
    update_field_action: Action<(String, String, String, FieldConfig), Result<(), ServerFnError>>,
) -> impl IntoView {
    let name = RwSignal::new(initial_name);
    let config = RwSignal::new(initial_config);

    let selected_type = move || {
        match config.get() {
            FieldConfig::Text(ref t) => match t {
                charac::models::field::TextConfig::SingleLine { .. } => "Text",
                charac::models::field::TextConfig::LongText { .. } => "Long Text",
                charac::models::field::TextConfig::Email => "Email",
                charac::models::field::TextConfig::URL => "URL",
                charac::models::field::TextConfig::Phone => "Phone",
            },
            FieldConfig::Number(ref n) => match n {
                charac::models::field::NumberConfig::Number { .. } => "Number",
                charac::models::field::NumberConfig::Decimal { .. } => "Decimal",
                charac::models::field::NumberConfig::Currency { .. } => "Currency",
                charac::models::field::NumberConfig::Percent { .. } => "Percent",
                charac::models::field::NumberConfig::Rating { .. } => "Rating",
            },
            FieldConfig::Datetime(_) => "Datetime",
            FieldConfig::Select(ref s) => match s {
                charac::models::field::SelectConfig::Single { .. } => "Select",
                charac::models::field::SelectConfig::Multi { .. } => "Select Multi",
            },
            _ => "Text",
        }
        .to_string()
    };

    view! {
        <Dialog>
            <DialogTrigger variant=ButtonVariant::Ghost size=ButtonSize::Sm class="p-1 h-auto">
                <Settings2 class="size-3 text-muted-foreground hover:text-foreground transition-colors" />
            </DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Edit Field"</DialogTitle>
                        <DialogDescription>
                            "Modify field settings. Note: changing types may be destructive."
                        </DialogDescription>
                    </DialogHeader>

                    <div class="flex flex-col gap-4 py-4">
                        <div class="flex flex-col gap-2">
                            <Label html_for="edit-field-name">"Name"</Label>
                            <Input id="edit-field-name" bind_value=name />
                        </div>
                        <div class="flex flex-col gap-2">
                            <Label>"Type"</Label>
                            <Select
                                default_value=selected_type()
                                on_change=Callback::new(move |val: Option<String>| {
                                    if let Some(v) = val {
                                        let new_config = match v.as_str() {
                                            "Text" => {
                                                FieldConfig::Text(charac::models::field::TextConfig::SingleLine {
                                                    default: None,
                                                    max_length: 255,
                                                })
                                            }
                                            "Long Text" => {
                                                FieldConfig::Text(charac::models::field::TextConfig::LongText {
                                                    rich_text: false,
                                                })
                                            }
                                            "Email" => {
                                                FieldConfig::Text(charac::models::field::TextConfig::Email)
                                            }
                                            "URL" => {
                                                FieldConfig::Text(charac::models::field::TextConfig::URL)
                                            }
                                            "Phone" => {
                                                FieldConfig::Text(charac::models::field::TextConfig::Phone)
                                            }
                                            "Number" => {
                                                FieldConfig::Number(charac::models::field::NumberConfig::Number {
                                                    default: None,
                                                })
                                            }
                                            "Decimal" => {
                                                FieldConfig::Number(charac::models::field::NumberConfig::Decimal {
                                                    default: None,
                                                    precision: 2,
                                                })
                                            }
                                            "Currency" => {
                                                FieldConfig::Number(charac::models::field::NumberConfig::Currency {
                                                    currency: "USD".to_string(),
                                                    precision: 2,
                                                })
                                            }
                                            "Percent" => {
                                                FieldConfig::Number(charac::models::field::NumberConfig::Percent {
                                                    precision: 2,
                                                    show_bar: false,
                                                })
                                            }
                                            "Rating" => {
                                                FieldConfig::Number(charac::models::field::NumberConfig::Rating {
                                                    max: 5,
                                                    icon_type: charac::models::field::RatingIcon::Star,
                                                    color: [255, 200, 0],
                                                })
                                            }
                                            "Datetime" => {
                                                FieldConfig::Datetime(charac::models::field::DatetimeConfig::Date {
                                                    format: charac::models::field::DateFormat::ISO,
                                                    include_time: true,
                                                })
                                            }
                                            _ => {
                                                FieldConfig::Text(charac::models::field::TextConfig::SingleLine {
                                                    default: None,
                                                    max_length: 255,
                                                })
                                            }
                                        };
                                        config.set(new_config);
                                    }
                                })
                            >
                                <SelectTrigger>
                                    <SelectValue placeholder="Select type..." />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectOption value="Text".to_string()>"Text"</SelectOption>
                                        <SelectOption value="Long Text"
                                            .to_string()>"Long Text"</SelectOption>
                                        <SelectOption value="Email"
                                            .to_string()>"Email"</SelectOption>
                                        <SelectOption value="URL".to_string()>"URL"</SelectOption>
                                        <SelectOption value="Phone"
                                            .to_string()>"Phone"</SelectOption>
                                        <SelectOption value="Number"
                                            .to_string()>"Number"</SelectOption>
                                        <SelectOption value="Decimal"
                                            .to_string()>"Decimal"</SelectOption>
                                        <SelectOption value="Currency"
                                            .to_string()>"Currency"</SelectOption>
                                        <SelectOption value="Percent"
                                            .to_string()>"Percent"</SelectOption>
                                        <SelectOption value="Rating"
                                            .to_string()>"Rating"</SelectOption>
                                        <SelectOption value="Datetime"
                                            .to_string()>"Datetime"</SelectOption>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>

                        // Config-specific editors
                        {move || {
                            let current_config = config.get();
                            match current_config {
                                FieldConfig::Number(n) => {
                                    match n {
                                        charac::models::field::NumberConfig::Rating { max, .. } => {
                                            let maxx = RwSignal::new(max.to_string());
                                            view! {
                                                <div class="flex flex-col gap-2 border-t pt-4">
                                                    <Label>"Max Rating (1-10)"</Label>
                                                    <Input
                                                        r#type=InputType::Number
                                                        bind_value=maxx
                                                        on:input=move |ev| {
                                                            if let Ok(m) = event_target_value(&ev).parse::<usize>() {
                                                                let m = m.clamp(1, 10);
                                                                config
                                                                    .update(|c| {
                                                                        if let FieldConfig::Number(
                                                                            charac::models::field::NumberConfig::Rating {
                                                                                max: old_max,
                                                                                ..
                                                                            },
                                                                        ) = c {
                                                                            *old_max = m;
                                                                        }
                                                                    });
                                                            }
                                                        }
                                                    />
                                                </div>
                                            }
                                                .into_any()
                                        }
                                        charac::models::field::NumberConfig::Decimal {
                                            precision,
                                            ..
                                        } => {
                                            let precisionn = RwSignal::new(precision.to_string());
                                            view! {
                                                <div class="flex flex-col gap-2 border-t pt-4">
                                                    <Label>"Precision"</Label>
                                                    <Input
                                                        r#type=InputType::Number
                                                        bind_value=precisionn
                                                        on:input=move |ev| {
                                                            if let Ok(p) = event_target_value(&ev).parse::<u8>() {
                                                                config
                                                                    .update(|c| {
                                                                        if let FieldConfig::Number(
                                                                            charac::models::field::NumberConfig::Decimal {
                                                                                precision: old_p,
                                                                                ..
                                                                            },
                                                                        ) = c {
                                                                            *old_p = p;
                                                                        }
                                                                    });
                                                            }
                                                        }
                                                    />
                                                </div>
                                            }
                                                .into_any()
                                        }
                                        _ => view! {}.into_any(),
                                    }
                                }
                                _ => view! {}.into_any(),
                            }
                        }}
                    </div>

                    <DialogFooter>
                        <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                        <Button
                            attr:r#type="button"
                            attr:disabled=move || update_field_action.pending().get()
                            on:click=move |_| {
                                update_field_action
                                    .dispatch((
                                        "".to_string(),
                                        field_id.clone(),
                                        name.get(),
                                        config.get(),
                                    ));
                            }
                        >
                            "Save Changes"
                        </Button>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn DeleteRecordDialog(
    table_id: String,
    record_id: String,
    delete_action: Action<(String, String), Result<(), ServerFnError>>,
) -> impl IntoView {
    let on_complete = Callback::new(move |_| {
        delete_action.dispatch((table_id.clone(), record_id.clone()));
    });

    view! {
        <Dialog>
            <DialogTrigger
                variant=ButtonVariant::Ghost
                size=ButtonSize::Sm
                class="w-full justify-start h-8 px-2 text-destructive hover:text-destructive hover:bg-destructive/10"
            >
                <Trash2 class="mr-2 size-4" />
                "Delete Record"
            </DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
                <DialogBody>
                    <DialogHeader>
                        <DialogTitle>"Delete Record"</DialogTitle>
                        <DialogDescription>
                            "This action cannot be undone. Please hold the button below to confirm."
                        </DialogDescription>
                    </DialogHeader>

                    <div class="flex justify-center py-8">
                        <ButtonAction
                            on_complete=on_complete
                            duration_ms=1500
                            variant=ButtonVariant::Destructive
                            class="w-full"
                        >
                            <Trash2 />
                            <span>"Hold to Delete"</span>
                        </ButtonAction>
                    </div>

                    <DialogFooter>
                        <DialogClose class="w-full">"Cancel"</DialogClose>
                    </DialogFooter>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}

#[component]
pub fn CellInput(
    #[prop(into)] initial_value: String,
    config: FieldConfig,
    #[prop(into)] on_change: Callback<String>,
) -> impl IntoView {
    // Local state — initialize once from props and then let the user own it
    let (val, set_val) = signal(initial_value);
    let (is_editing, set_is_editing) = signal(false);

    let input_type = match &config {
        FieldConfig::Number(num_config) => match num_config {
            charac::models::field::NumberConfig::Decimal { .. } => "number",
            charac::models::field::NumberConfig::Number { .. } => "number",
            charac::models::field::NumberConfig::Currency { .. } => "number",
            charac::models::field::NumberConfig::Percent { .. } => "number",
            charac::models::field::NumberConfig::Rating { .. } => "number",
        },
        FieldConfig::Datetime(_) => "datetime-local",
        FieldConfig::Text(text_config) => match text_config {
            charac::models::field::TextConfig::Email => "email",
            charac::models::field::TextConfig::URL => "url",
            charac::models::field::TextConfig::Phone => "tel",
            _ => "text",
        },
        _ => "text",
    };

    let is_long_text = matches!(
        config,
        FieldConfig::Text(charac::models::field::TextConfig::LongText { .. })
    );

    let display_config = config.clone();
    let display_value = move || {
        let v = val.get();
        if v.is_empty() {
            return "".to_string();
        }
        match &display_config {
            FieldConfig::Number(n) => match n {
                charac::models::field::NumberConfig::Percent { .. } => format!("{}%", v),
                charac::models::field::NumberConfig::Currency { .. } => {
                    if v.contains('$') || v.contains('€') {
                        v
                    } else {
                        format!("$ {}", v)
                    }
                }
                _ => v,
            },
            _ => v,
        }
    };

    view! {
        <div
            class="w-full h-full min-h-[32px] flex items-center px-1"
            on:click=move |_| set_is_editing.set(true)
        >
            {move || {
                if is_editing.get() {
                    match &config {
                        FieldConfig::Select(select_config) => {
                            match select_config {
                                charac::models::field::SelectConfig::Single { options } => {
                                    let options = options.clone();
                                    view! {
                                        <Select
                                            default_value=val.get()
                                            on_change=Callback::new(move |new_val: Option<String>| {
                                                if let Some(v) = new_val {
                                                    set_val.set(v.clone());
                                                    on_change.run(v);
                                                }
                                                set_is_editing.set(false);
                                            })
                                        >
                                            <SelectTrigger class="h-8 border-none bg-transparent hover:bg-muted/50 w-full">
                                                <SelectValue placeholder="Select..." />
                                            </SelectTrigger>
                                            <SelectContent>
                                                <SelectGroup>
                                                    {options
                                                        .into_iter()
                                                        .map(|opt| {
                                                            view! {
                                                                <SelectOption value=opt.label.clone()>
                                                                    {opt.label}
                                                                </SelectOption>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </SelectGroup>
                                            </SelectContent>
                                        </Select>
                                    }
                                        .into_any()
                                }
                                _ => {
                                    view! {
                                        <p class="text-xs text-muted-foreground">
                                            "Multi-select not supported"
                                        </p>
                                    }
                                        .into_any()
                                }
                            }
                        }
                        _ if is_long_text => {
                            view! {
                                <textarea
                                    autofocus
                                    class="bg-background border rounded p-1 w-full text-sm min-h-[60px] resize-y focus:outline-none focus:ring-1 focus:ring-primary"
                                    on:click=move |ev| ev.stop_propagation()
                                    on:input=move |ev| set_val.set(event_target_value(&ev))
                                    on:blur=move |_| {
                                        on_change.run(val.get());
                                        set_is_editing.set(false);
                                    }
                                    prop:value=move || val.get()
                                />
                            }
                                .into_any()
                        }
                        _ => {
                            view! {
                                <input
                                    autofocus
                                    type=input_type
                                    class="bg-background border rounded p-1 w-full text-sm h-8 focus:outline-none focus:ring-1 focus:ring-primary"
                                    on:click=move |ev| ev.stop_propagation()
                                    on:input=move |ev| set_val.set(event_target_value(&ev))
                                    on:blur=move |_| {
                                        on_change.run(val.get());
                                        set_is_editing.set(false);
                                    }
                                    on:keydown=move |ev| {
                                        if ev.key() == "Enter" {
                                            on_change.run(val.get());
                                            set_is_editing.set(false);
                                        }
                                    }
                                    prop:value=move || val.get()
                                />
                            }
                                .into_any()
                        }
                    }
                } else {
                    view! {
                        <div class="truncate text-sm w-full cursor-text select-none">
                            {display_value()}
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
