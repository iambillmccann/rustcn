use dioxus::prelude::*;
use dioxus_router::{Route, Router};

#[component]
pub fn App() -> Element {
    render! {
        Router {
            Route { to: "/", component: home::Home },
            Route { to: "/about", component: about::About },
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
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

#[component]
pub fn About() -> Element {
    render! {
        div {
            class: "p-4",
            "About Page"
        }
    }
}
