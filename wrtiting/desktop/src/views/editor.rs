

    // client/components/layout/app_layout.rs
use dioxus::prelude::*;


#[component]
pub fn Editor() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: " flex-1",
                div { class: "mx-auto px-8 py-6", "Editor content goes here" }
            }
        }
    }
}