use dioxus::prelude::*;

use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(MobileNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

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


// #[component]
// fn MobileNavbar() -> Element {
//     rsx! {
//         Navbar {
//             Link { to: Route::Home {}, "Home" }
//             Link { to: Route::Blog { id: 1 }, "Blog" }
//         }

//         Outlet::<Route> {}
//     }
// }
