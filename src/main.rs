use dioxus::prelude::*;
mod app;

fn main() {
    dioxus::web::launch(app::App);
}
