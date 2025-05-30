

    // client/components/layout/app_layout.rs
use dioxus::prelude::*;

#[component]
pub fn HelpMain() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: " flex-1",
                div { class: "mx-auto px-8 py-6", "HelpMain content goes here" }
            }
        }
    }
}


#[component]
pub fn HelpFaq() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: " flex-1",
                div { class: "mx-auto px-8 py-6", "HelpFAQ content goes here" }
            }
        }
    }
}

#[component]
pub fn HelpContact() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: " flex-1",
                div { class: "mx-auto px-8 py-6", "HelpContact content goes here" }
            }
        }
    }
}