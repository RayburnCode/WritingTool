

    // client/components/layout/app_layout.rs
use dioxus::prelude::*;


#[component]
pub fn AccountSettings() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            main { class: " flex-1",
                div { class: "mx-auto px-8 py-6", "Account Settings content goes here" }
            }
            p { "Settings" }
            ul {
                li { "Change Password" }
                li { "2 Factor Auth" }
                li { "Notifications, SMS, In app, email" }
                li { "Connected apps. Gmail and GHL" }
                li { "Custom color settings?" }
            }
        }
    }
}