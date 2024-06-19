use crate::components::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

pub fn HomePage() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            "Hello World"
            Button {
                text: "Click Me",
                variant: Some(ButtonVariant::Default),
                size: Some(ButtonSize::Default),
                on_click: move |_| {
                    println!("Button clicked!");
                }
            },
            Button {
                text: "Delete",
                variant: Some(ButtonVariant::Destructive),
                size: Some(ButtonSize::Sm),
                on_click: move |_| {
                    println!("Destructive button clicked!");
                }
            }
        }
    }
}
