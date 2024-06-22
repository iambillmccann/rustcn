use crate::components::{
    Alert, AlertDescription, AlertTitle, AlertVariant, Button, ButtonSize, ButtonVariant,
};
use dioxus::prelude::*;

#[component]
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
                        Alert {
                            variant: Some(AlertVariant::Default),
                            AlertTitle { "Alert" }
                            AlertDescription { "You clicked on me." }
                        }
                    }
                } else {
                    rsx! { "" }
                }
            }
        }
    }
}
