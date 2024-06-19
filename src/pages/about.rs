use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "p-4",
            "About Page"
        }
    }
}
