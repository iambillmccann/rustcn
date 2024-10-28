use dioxus::prelude::*;
use std::collections::HashMap;

// Form context to manage form state
#[derive(Clone, Debug)]
pub struct FormState {
    values: HashMap<String, String>,
    errors: HashMap<String, String>,
    touched: HashMap<String, bool>,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            errors: HashMap::new(),
            touched: HashMap::new(),
        }
    }
}

#[derive(Props)]
pub struct FormProps {
    #[props(optional)]
    class: Option<String>,
    children: Element,
    on_submit: EventHandler<FormState>,
}

#[component]
pub fn Form(cx: Scope<FormProps>) -> Element {
    let form_state = use_ref(cx, FormState::new);

    cx.render(rsx! {
        form {
            class: cx.props.class.clone(),
            onsubmit: move |event| {
                event.prevent_default();
                cx.props.on_submit.call((*form_state.read()).clone());
            },
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct FormFieldProps {
    #[props(optional)]
    class: Option<String>,
    name: String,
    children: Element,
}

#[component]
pub fn FormField(cx: Scope<FormFieldProps>) -> Element {
    cx.render(rsx! {
        div {
            class: cx.props.class.clone(),
            "data-field": cx.props.name.clone(),
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct FormLabelProps {
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    for_input: Option<String>,
    children: Element,
}

#[component]
pub fn FormLabel(cx: Scope<FormLabelProps>) -> Element {
    cx.render(rsx! {
        label {
            class: {
                let mut classes = vec![];
                if let Some(class) = &cx.props.class {
                    classes.push(class.as_str());
                }
                classes.join(" ")
            },
            r#for: cx.props.for_input.clone(),
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct FormControlProps {
    #[props(optional)]
    class: Option<String>,
    children: Element,
}

#[component]
pub fn FormControl(cx: Scope<FormControlProps>) -> Element {
    cx.render(rsx! {
        div {
            class: {
                let mut classes = vec!["form-control"];
                if let Some(class) = &cx.props.class {
                    classes.push(class.as_str());
                }
                classes.join(" ")
            },
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct FormDescriptionProps {
    #[props(optional)]
    class: Option<String>,
    children: Element,
}

#[component]
pub fn FormDescription(cx: Scope<FormDescriptionProps>) -> Element {
    cx.render(rsx! {
        p {
            class: {
                let mut classes = vec!["text-sm", "text-muted-foreground"];
                if let Some(class) = &cx.props.class {
                    classes.push(class.as_str());
                }
                classes.join(" ")
            },
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct FormMessageProps {
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    error: Option<String>,
    children: Element,
}

#[component]
pub fn FormMessage(cx: Scope<FormMessageProps>) -> Element {
    let error_message = cx.props.error.clone();

    if error_message.is_none() && cx.props.children.is_none() {
        return None;
    }

    cx.render(rsx! {
        p {
            class: {
                let mut classes = vec!["text-sm", "font-medium", "text-destructive"];
                if let Some(class) = &cx.props.class {
                    classes.push(class.as_str());
                }
                classes.join(" ")
            },
            {error_message.unwrap_or_default()},
            &cx.props.children
        }
    })
}
