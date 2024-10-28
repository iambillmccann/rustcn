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

// Form provider component
#[derive(Props)]
pub struct FormProps<'a> {
    #[props(optional)]
    class: Option<&'a str>,
    children: Element<'a>,
    on_submit: EventHandler<'a, FormState>,
}

pub fn Form<'a>(cx: Scope<'a, FormProps<'a>>) -> Element {
    let form_state = use_ref(cx, FormState::new);

    cx.render(rsx! {
        form {
            class: cx.props.class,
            onsubmit: move |event| {
                event.prevent_default();
                cx.props.on_submit.call((*form_state.read()).clone());
            },
            &cx.props.children
        }
    })
}

// Form field component
#[derive(Props)]
pub struct FormFieldProps<'a> {
    #[props(optional)]
    class: Option<&'a str>,
    name: &'a str,
    children: Element<'a>,
}

pub fn FormField<'a>(cx: Scope<'a, FormFieldProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: cx.props.class,
            "data-field": cx.props.name,
            &cx.props.children
        }
    })
}

// Form label component
#[derive(Props)]
pub struct FormLabelProps<'a> {
    #[props(optional)]
    class: Option<&'a str>,
    #[props(optional)]
    for_input: Option<&'a str>,
    children: Element<'a>,
}

pub fn FormLabel<'a>(cx: Scope<'a, FormLabelProps<'a>>) -> Element {
    cx.render(rsx! {
        label {
            class: {
                let mut classes = vec![];
                if let Some(class) = cx.props.class {
                    classes.push(class);
                }
                classes.join(" ")
            },
            r#for: cx.props.for_input,
            &cx.props.children
        }
    })
}

// Form control component
#[derive(Props)]
pub struct FormControlProps<'a> {
    #[props(optional)]
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn FormControl<'a>(cx: Scope<'a, FormControlProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: {
                let mut classes = vec!["form-control"];
                if let Some(class) = cx.props.class {
                    classes.push(class);
                }
                classes.join(" ")
            },
            &cx.props.children
        }
    })
}

// Form description component
#[derive(Props)]
pub struct FormDescriptionProps<'a> {
    #[props(optional)]
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn FormDescription<'a>(cx: Scope<'a, FormDescriptionProps<'a>>) -> Element {
    cx.render(rsx! {
        p {
            class: {
                let mut classes = vec!["text-sm", "text-muted-foreground"];
                if let Some(class) = cx.props.class {
                    classes.push(class);
                }
                classes.join(" ")
            },
            &cx.props.children
        }
    })
}

// Form error message component
#[derive(Props)]
pub struct FormMessageProps<'a> {
    #[props(optional)]
    class: Option<&'a str>,
    #[props(optional)]
    error: Option<&'a str>,
    children: Element<'a>,
}

pub fn FormMessage<'a>(cx: Scope<'a, FormMessageProps<'a>>) -> Element {
    let error_message = cx.props.error.map(|err| err.to_string());

    if error_message.is_none() && cx.props.children.is_none() {
        return None;
    }

    cx.render(rsx! {
        p {
            class: {
                let mut classes = vec!["text-sm", "font-medium", "text-destructive"];
                if let Some(class) = cx.props.class {
                    classes.push(class);
                }
                classes.join(" ")
            },
            error_message.unwrap_or_else(|| String::new()),
            &cx.props.children
        }
    })
}
