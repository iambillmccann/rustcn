use crate::components::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay, AlertDialogTitle, AlertDialogTrigger,
    Button, ButtonSize, ButtonVariant, Textarea,
};
use dioxus::prelude::*;

pub fn HomePage() -> Element {
    let mut show_alert = use_signal(|| false);
    let mut textarea_value = use_signal(|| String::from(""));

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
                },
                class: "border border-white"
            },
            Textarea {
                class: Some(String::from("my-custom-class")),
                value: Some(textarea_value.read().clone()),
                placeholder: Some(String::from("Enter text here...")),
                on_input: move |event: FormEvent| {
                    textarea_value.set(event.value().clone());
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
                                    AlertDialogDescription { "{textarea_value.read()}" }
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
