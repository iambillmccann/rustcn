use dioxus::events::MouseEvent;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogProps {
    children: Element,
}

#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    rsx! {
        div {
            class: "alert-dialog",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTriggerProps {
    children: Element,
    #[props(optional)]
    on_click: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn AlertDialogTrigger(props: AlertDialogTriggerProps) -> Element {
    rsx! {
        button {
            class: "alert-dialog-trigger",
            onclick: move |e| {
                if let Some(on_click) = &props.on_click {
                    on_click.call(e);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogOverlayProps {
    children: Element,
}

#[component]
pub fn AlertDialogOverlay(props: AlertDialogOverlayProps) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/80",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    children: Element,
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    rsx! {
        div {
            class: "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-white p-6 shadow-lg duration-200 sm:rounded-lg",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogHeaderProps {
    children: Element,
}

#[component]
pub fn AlertDialogHeader(props: AlertDialogHeaderProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-2 text-center sm:text-left",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogFooterProps {
    children: Element,
}

#[component]
pub fn AlertDialogFooter(props: AlertDialogFooterProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTitleProps {
    children: Element,
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    rsx! {
        h2 {
            class: "text-lg font-semibold",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    children: Element,
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! {
        p {
            class: "text-sm text-muted-foreground",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    children: Element,
    #[props(optional)]
    on_click: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        button {
            class: "alert-dialog-action bg-blue-500 text-white px-4 py-2 rounded",
            onclick: move |e| {
                if let Some(on_click) = &props.on_click {
                    on_click.call(e);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    children: Element,
    #[props(optional)]
    on_click: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        button {
            class: "alert-dialog-cancel bg-gray-300 text-black px-4 py-2 rounded mt-2 sm:mt-0",
            onclick: move |e| {
                if let Some(on_click) = &props.on_click {
                    on_click.call(e);
                }
            },
            {props.children}
        }
    }
}
