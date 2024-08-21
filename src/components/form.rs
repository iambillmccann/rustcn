use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct FormState {
    pub errors: HashMap<String, String>,
}

#[derive(Props)]
pub struct FormProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    on_submit: Option<EventHandler<'a, FormEvent>>,
}

#[component]
pub fn Form<'a>(props: FormProps<'a>) -> Element<'a> {
    let form_state = use_context::<Arc<FormState>>().unwrap_or_else(|| {
        Arc::new(FormState {
            errors: HashMap::new(),
        })
    });

    rsx! {
        form {
            onsubmit: move |e| {
                e.prevent_default();
                if let Some(on_submit) = &props.on_submit {
                    on_submit.call(e);
                }
            },
            &props.children
        }
    }
}

#[component]
pub fn FormItem<'a>(props: Element<'a>) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            &props
        }
    }
}

#[component]
pub fn FormLabel<'a>(props: Element<'a>) -> Element {
    rsx! {
        label {
            class: "text-black",
            &props
        }
    }
}

#[component]
pub fn FormControl<'a>(props: Element<'a>) -> Element {
    rsx! {
        div {
            class: "form-control",
            &props
        }
    }
}

#[component]
pub fn FormDescription<'a>(props: Element<'a>) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground",
            &props
        }
    }
}

#[component]
pub fn FormMessage<'a>(props: Element<'a>) -> Element {
    let form_state = use_context::<Arc<FormState>>().unwrap_or_else(|| {
        Arc::new(FormState {
            errors: HashMap::new(),
        })
    });

    let error_message = form_state
        .errors
        .get(&props.id.unwrap_or_default())
        .unwrap_or(&String::new());

    if error_message.is_empty() {
        return rsx! { "" };
    }

    rsx! {
        p {
            class: "text-sm font-medium text-destructive",
            "{error_message}"
        }
    }
}
