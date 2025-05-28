use dioxus::prelude::*;

use ui::Navbar;
use views::{Blog, Home, TestingView};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    //    #[layout(AppLayout)]

    #[layout(DesktopNavbar)]

    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/testing")]
    TestingView {},
}

// Update the path if your CSS file is located elsewhere, e.g. "/assets/css/tailwind.css"
const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
            Link { to: Route::TestingView {}, "Testing" }
        }

        Outlet::<Route> {}
    }
}
