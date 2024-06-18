use crate::components::Button;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    render! {
        div {
            class: "p-4",
            "Home Page",
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
