

    // client/components/layout/app_layout.rs
use dioxus::prelude::*;


#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: " flex-1",
                div { class: "mx-auto px-8 py-6", "Profile content goes here" }
            }

            p { " ability to change profile info" }
            ul {
                li { "Google search: example pages for profile information" }
                li { "Name" }
                li { "email" }
                li { "Phone" }
                li { "Address" }
                li { "Profile photo" }
            }
        }
    }
}