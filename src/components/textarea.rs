use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    value: Option<String>,
    #[props(optional)]
    placeholder: Option<String>,
    #[props(optional)]
    disabled: Option<bool>,
    #[props(optional)]
    on_input: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let TextareaProps {
        class,
        value,
        placeholder,
        disabled,
        on_input,
    } = props;

    let mut base_class = String::from(
        "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
    );

    if let Some(custom_class) = class {
        base_class.push_str(&format!(" {}", custom_class));
    }

    rsx! {
        textarea {
            class: "{base_class}",
            value: "{value.unwrap_or_default()}",
            placeholder: "{placeholder.unwrap_or_default()}",
            disabled: "{disabled.unwrap_or(false)}",
            oninput: move |event| {
                if let Some(on_input) = &on_input {
                    on_input.call(event);
                }
            }
        }
    }
}
