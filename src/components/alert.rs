use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AlertVariant {
    Default,
    Destructive,
}

impl AlertVariant {
    pub fn to_class(&self) -> &str {
        match self {
            AlertVariant::Default => "bg-background text-foreground",
            AlertVariant::Destructive => "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive",
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    #[props(optional)]
    variant: Option<AlertVariant>,
    #[props(optional)]
    class: Option<&'static str>,
    children: Element,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let variant_class = {
        let variant = props.variant.unwrap_or(AlertVariant::Default);
        variant.to_class().to_string()
    };
    let additional_class = props.class.unwrap_or("").to_string();

    rsx! {
        div {
            class: format!("relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground {} {}", variant_class, additional_class),
            role: "alert",
            {props.children},
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertTitleProps {
    #[props(optional)]
    class: Option<&'static str>,
    children: Element,
}

#[component]
pub fn AlertTitle(props: AlertTitleProps) -> Element {
    let additional_class = props.class.unwrap_or("").to_string();

    rsx! {
        h5 {
            class: format!("mb-1 font-medium leading-none tracking-tight {}", additional_class),
            {props.children},
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertDescriptionProps {
    #[props(optional)]
    class: Option<&'static str>,
    children: Element,
}

#[component]
pub fn AlertDescription(props: AlertDescriptionProps) -> Element {
    let additional_class = props.class.unwrap_or("").to_string();

    rsx! {
        div {
            class: format!("text-sm [&_p]:leading-relaxed {}", additional_class),
            {props.children},
        }
    }
}
