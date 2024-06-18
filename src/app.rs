use dioxus::prelude::*;
use dioxus_router::{Route, Router};
mod about;
mod home;

#[component]
pub fn App() -> Element {
    render! {
        Router {
            Route { to: "/", component: home::Home },
            Route { to: "/about", component: about::About },
        }
    }
}
