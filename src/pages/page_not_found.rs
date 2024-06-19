// src/pages/landing.rs

use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            "Oops! The page {route:?} not found."
        }
    }
}
