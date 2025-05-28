

    // client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::Route;
use super::Footer;
use super::DesktopNavbar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            DesktopNavbar {}
            main { class: " flex-1",
                div { class: "mx-auto px-8 py-6", Outlet::<Route> {} }
            }
            Footer {}
        }
    }
}