use dioxus::prelude::*;
use std::collections::HashMap;

// Utility function similar to cn() from shadcn
fn cx(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

// Form context types
#[derive(Clone, Debug, PartialEq)]
pub struct FormFieldContext {
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormItemContext {
    id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormState {
    pub errors: HashMap<String, String>,
}

// Form Provider Component
#[derive(Props)]
pub struct FormProps {
    children: Element,
}

#[component]
pub fn Form(cx: Scope, props: FormProps) -> Element {
    let form_state = use_shared_state_provider(cx, || FormState {
        errors: HashMap::new(),
    });

    render! {
        form {
            onsubmit: move |event| {
                event.prevent_default();
            },
            {props.children}
        }
    }
}

// Form Field Component
#[derive(Props)]
pub struct FormFieldProps {
    name: String,
    children: Element,
}

#[component]
pub fn FormField(cx: Scope, props: FormFieldProps) -> Element {
    let context = FormFieldContext {
        name: props.name.clone(),
    };
    use_shared_state_provider(cx, || context);

    render! {
        div {
            {props.children}
        }
    }
}

// Form Item Component
#[derive(Props)]
pub struct FormItemProps {
    #[props(default = "")]
    class: String,
    children: Element,
}

#[component]
pub fn FormItem(cx: Scope, props: FormItemProps) -> Element {
    let id = use_state(cx, || uuid::Uuid::new_v4().to_string());
    let context = FormItemContext {
        id: id.get().clone(),
    };
    use_shared_state_provider(cx, || context);

    render! {
        div {
            class: cx(&["space-y-2", &props.class]),
            {props.children}
        }
    }
}

// Form Label Component
#[derive(Props)]
pub struct FormLabelProps {
    #[props(default = "")]
    class: String,
    children: Element,
}

#[component]
pub fn FormLabel(cx: Scope, props: FormLabelProps) -> Element {
    let form_field = use_shared_state::<FormFieldContext>(cx).unwrap();
    let form_state = use_shared_state::<FormState>(cx).unwrap();

    let has_error = form_state
        .read()
        .errors
        .contains_key(&form_field.read().name);
    let error_class = if has_error { "text-destructive" } else { "" };

    render! {
        label {
            class: cx(&[error_class, &props.class]),
            {props.children}
        }
    }
}

// Form Control Component
#[derive(Props)]
pub struct FormControlProps {
    children: Element,
}

#[component]
pub fn FormControl(cx: Scope, props: FormControlProps) -> Element {
    let form_field = use_shared_state::<FormFieldContext>(cx).unwrap();
    let form_item = use_shared_state::<FormItemContext>(cx).unwrap();
    let form_state = use_shared_state::<FormState>(cx).unwrap();

    let has_error = form_state
        .read()
        .errors
        .contains_key(&form_field.read().name);
    let description_id = format!("{}-description", form_item.read().id);
    let message_id = format!("{}-message", form_item.read().id);

    let aria_describedby = if has_error {
        format!("{} {}", description_id, message_id)
    } else {
        description_id.clone()
    };

    render! {
        div {
            id: form_item.read().id.clone(),
            aria_describedby: aria_describedby,
            aria_invalid: has_error.to_string(),
            {props.children}
        }
    }
}

// Form Description Component
#[derive(Props)]
pub struct FormDescriptionProps {
    #[props(default = "")]
    class: String,
    children: Element,
}

#[component]
pub fn FormDescription(cx: Scope, props: FormDescriptionProps) -> Element {
    let form_item = use_shared_state::<FormItemContext>(cx).unwrap();
    let description_id = format!("{}-description", form_item.read().id);

    render! {
        p {
            id: description_id,
            class: cx(&["text-sm text-muted-foreground", &props.class]),
            {props.children}
        }
    }
}

// Form Message Component
#[derive(Props)]
pub struct FormMessageProps {
    #[props(default = "")]
    class: String,
    children: Element,
}

#[component]
pub fn FormMessage(cx: Scope, props: FormMessageProps) -> Element {
    let form_field = use_shared_state::<FormFieldContext>(cx).unwrap();
    let form_item = use_shared_state::<FormItemContext>(cx).unwrap();
    let form_state = use_shared_state::<FormState>(cx).unwrap();

    let error_message = form_state
        .read()
        .errors
        .get(&form_field.read().name)
        .cloned();
    let message_id = format!("{}-message", form_item.read().id);

    // Return null if no error and no children
    if error_message.is_none() && props.children.is_none() {
        return None;
    }

    render! {
        p {
            id: message_id,
            class: cx(&["text-sm font-medium text-destructive", &props.class]),
            {error_message.unwrap_or_else(|| props.children.unwrap_or_default())}
        }
    }
}
