use crate::components::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay, AlertDialogTitle, AlertDialogTrigger,
    Button, ButtonSize, ButtonVariant,
};
use dioxus::prelude::*;

pub fn HomePage() -> Element {
    let mut show_alert = use_signal(|| false);

    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl flex-col",
            "Hello World",
            Button {
                text: "Click Me",
                variant: Some(ButtonVariant::Default),
                size: Some(ButtonSize::Default),
                on_click: move |_| {
                    show_alert.set(true);
                    println!("Button clicked!");
                }
            },
            {
                if *show_alert.read() {
                    rsx! {
                        AlertDialog {
                            AlertDialogTrigger { "Open Dialog" }
                            AlertDialogOverlay {
                                AlertDialogContent {
                                    AlertDialogHeader { AlertDialogTitle { "Alert" } }
                                    AlertDialogDescription { "You clicked on me." }
                                    AlertDialogFooter {
                                        AlertDialogAction {
                                            on_click: move |_| show_alert.set(false),
                                            "Confirm"
                                        }
                                        AlertDialogCancel {
                                            on_click: move |_| show_alert.set(false),
                                            "Cancel"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    rsx! { "" }
                }
            }
        }
    }
}
