use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Props, PartialEq)]
pub struct FormProps {
    children: Element,
    #[props(optional)]
    on_submit: Option<EventHandler<FormEvent>>,
}

#[derive(Props, PartialEq)]
pub struct FormProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    on_submit: Option<EventHandler<'a, FormEvent>>,
}

#[component]
pub fn Form(props: FormProps) -> Element {
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
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormItemProps {
    children: Element,
}

#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormLabelProps {
    children: Element,
    #[props(optional)]
    html_for: Option<String>,
}

#[component]
pub fn FormLabel(props: FormLabelProps) -> Element {
    rsx! {
        label {
            class: "text-black",
            html_for: "{props.html_for.unwrap_or_default()}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormControlProps {
    children: Element,
}

#[component]
pub fn FormControl(props: FormControlProps) -> Element {
    rsx! {
        div {
            class: "form-control",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormDescriptionProps {
    children: Element,
}

#[component]
pub fn FormDescription(props: FormDescriptionProps) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq)]
pub struct FormMessageProps {
    children: Element,
}

#[component]
pub fn FormMessage(props: FormMessageProps) -> Element {
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
