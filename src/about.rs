use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    render! {
        div {
            class: "p-4",
            "About Page"
        }
    }
}
