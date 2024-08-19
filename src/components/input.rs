use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    value: Option<String>,
    #[props(optional)]
    placeholder: Option<String>,
    #[props(optional)]
    disabled: Option<bool>,
    #[props(optional)]
    input_type: Option<String>,
    #[props(optional)]
    on_input: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let InputProps {
        class,
        value,
        placeholder,
        disabled,
        input_type,
        on_input,
    } = props;

    let mut base_class = String::from(
        "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
    );

    if let Some(custom_class) = class {
        base_class.push_str(&format!(" {}", custom_class));
    }

    rsx! {
        input {
            class: "{base_class}",
            value: "{value.unwrap_or_default()}",
            placeholder: "{placeholder.unwrap_or_default()}",
            disabled: "{disabled.unwrap_or(false)}",
            r#type: "{input_type.unwrap_or_else(|| String::from("text"))}",
            oninput: move |event| {
                if let Some(on_input) = &on_input {
                    on_input.call(event);
                }
            }
        }
    }
}
