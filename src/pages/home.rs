use crate::components::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay, AlertDialogTitle, AlertDialogTrigger,
    Button, ButtonSize, ButtonVariant, Form, FormControl, FormItem, FormLabel, Input, Textarea,
};
use dioxus::prelude::*;

pub fn HomePage() -> Element {
    let mut show_alert = use_signal(|| false);
    let mut textarea_value = use_signal(|| String::from(""));
    let mut input_value = use_signal(|| String::from(""));

    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl flex-col",
            "Hello World",
            Form {
                on_submit: move |_| {
                    show_alert.set(true);
                },
                FormItem {
                    FormLabel {
                        "Title"
                    }
                    FormControl {
                        Input {
                            class: Some(String::from("my-custom-class text-black")),
                            value: Some(input_value.read().clone()),
                            placeholder: Some(String::from("Enter title here...")),
                            on_input: move |event: FormEvent| {
                                input_value.set(event.value().clone());
                            }
                        }
                    }
                },
                FormItem {
                    FormLabel {
                        "Description"
                    }
                    FormControl {
                        Textarea {
                            class: Some(String::from("my-custom-class text-black mt-4")),
                            value: Some(textarea_value.read().clone()),
                            placeholder: Some(String::from("Enter text here...")),
                            on_input: move |event: FormEvent| {
                                textarea_value.set(event.value().clone());
                            }
                        }
                    }
                },
                Button {
                    text: "Submit",
                    variant: Some(ButtonVariant::Default),
                    size: Some(ButtonSize::Default),
                    on_click: move |_| {
                        // This will trigger the form submission event
                        show_alert.set(true);
                        println!("Form submitted!");
                    },
                    class: "border border-white mt-4"
                },
            },
            {
                if *show_alert.read() {
                    rsx! {
                        AlertDialog {
                            AlertDialogTrigger { "Open Dialog" }
                            AlertDialogOverlay {
                                AlertDialogContent {
                                    AlertDialogHeader {
                                        div {
                                            class: "text-black",
                                            AlertDialogTitle {
                                                "{input_value.read()}"
                                            }
                                        }
                                    }
                                    AlertDialogDescription {
                                        div {
                                            class: "text-black",
                                            "{textarea_value.read()}"
                                        }
                                    }
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
