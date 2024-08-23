use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct FormState {
    pub errors: HashMap<String, String>,
}

#[derive(Props, PartialEq)]
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

#[derive(Props, PartialEq)]
pub struct FormItemProps<'a> {
    children: Element<'a>,
}

#[component]
pub fn FormItem<'a>(props: FormItemProps<'a>) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormLabelProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    html_for: Option<String>,
}

#[component]
pub fn FormLabel<'a>(props: FormLabelProps<'a>) -> Element {
    rsx! {
        label {
            class: "text-black",
            html_for: "{props.html_for.unwrap_or_default()}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormControlProps<'a> {
    children: Element<'a>,
}

#[component]
pub fn FormControl<'a>(props: FormControlProps<'a>) -> Element {
    rsx! {
        div {
            class: "form-control",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormDescriptionProps<'a> {
    children: Element<'a>,
}

#[component]
pub fn FormDescription<'a>(props: FormDescriptionProps<'a>) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormMessageProps<'a> {
    children: Element<'a>,
}

#[component]
pub fn FormMessage<'a>(props: FormMessageProps<'a>) -> Element {
    let form_state = use_context::<Arc<FormState>>().unwrap_or_else(|| {
        Arc::new(FormState {
            errors: HashMap::new(),
        })
    });

    let error_message = form_state
        .errors
        .get(&props.children.text())
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
