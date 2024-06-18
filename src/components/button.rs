use dioxus::prelude::*;

#[derive(PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn to_class(&self) -> &str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

#[derive(PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl ButtonSize {
    pub fn to_class(&self) -> &str {
        match self {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Sm => "h-9 rounded-md px-3",
            ButtonSize::Lg => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        }
    }
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    text: &'a str,
    #[props(optional)]
    variant: Option<ButtonVariant>,
    #[props(optional)]
    size: Option<ButtonSize>,
    #[props(optional)]
    class: Option<&'a str>,
    #[props(optional)]
    on_click: Option<EventHandler<'a, MouseEvent>>,
}

#[component]
pub fn Button<'a>(props: ButtonProps<'a>) -> Element {
    let variant_class = props.variant.unwrap_or(ButtonVariant::Default).to_class();
    let size_class = props.size.unwrap_or(ButtonSize::Default).to_class();
    let additional_class = props.class.unwrap_or("");

    render! {
        button {
            class: format_args!("inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {} {} {}", variant_class, size_class, additional_class),
            onclick: move |_| {
                if let Some(on_click) = &props.on_click {
                    on_click.call(());
                }
            },
            "{props.text}"
        }
    }
}
