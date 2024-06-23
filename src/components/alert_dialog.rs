use dioxus::events::MouseEvent;
use dioxus::prelude::*;

#[derive(Props)]
pub struct AlertDialogProps<'a> {
    children: Element<'a>,
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

#[derive(Props)]
pub struct AlertDialogTriggerProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    on_click: Option<EventHandler<'a, MouseEvent>>,
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

#[derive(Props)]
pub struct AlertDialogOverlayProps<'a> {
    children: Element<'a>,
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

#[derive(Props)]
pub struct AlertDialogContentProps<'a> {
    children: Element<'a>,
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    rsx! {
        div {
            class: "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 sm:rounded-lg",
            {props.children}
        }
    }
}

#[derive(Props)]
pub struct AlertDialogHeaderProps<'a> {
    children: Element<'a>,
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

#[derive(Props)]
pub struct AlertDialogFooterProps<'a> {
    children: Element<'a>,
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

#[derive(Props)]
pub struct AlertDialogTitleProps<'a> {
    children: Element<'a>,
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

#[derive(Props)]
pub struct AlertDialogDescriptionProps<'a> {
    children: Element<'a>,
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

#[derive(Props)]
pub struct AlertDialogActionProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    on_click: Option<EventHandler<'a, MouseEvent>>,
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        button {
            class: "alert-dialog-action",
            onclick: move |e| {
                if let Some(on_click) = &props.on_click {
                    on_click.call(e);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props)]
pub struct AlertDialogCancelProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    on_click: Option<EventHandler<'a, MouseEvent>>,
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        button {
            class: "alert-dialog-cancel mt-2 sm:mt-0",
            onclick: move |e| {
                if let Some(on_click) = &props.on_click {
                    on_click.call(e);
                }
            },
            {props.children}
        }
    }
}
