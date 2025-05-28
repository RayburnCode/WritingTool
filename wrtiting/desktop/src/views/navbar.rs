use crate::Route;
use dioxus::prelude::*;
// const LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub fn DesktopNavbar(children: Element) -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };

    rsx! {
        nav { id: "navbar", class: "w-full  text-white shadow-md",
            div { class: "px-8 py-2 mx-auto flex items-center justify-between",

                // // Left side: Logo
                // a { href: "https://rayburnlp.com",
                //     img { class: "h-15", src: LOGO, alt: "Card header image" }
                // }


                // Right side: Links and Button
                div { class: "flex gap-6 items-center text-sm text-gray-900",
                    Link {
                        to: Route::About {},
                        class: "hover:text-blue-400 transition",
                        "About"
                    }
                    Link {
                        to: Route::Team {},
                        class: "hover:text-blue-400 transition",
                        "Team"
                    }
                    Link {
                        to: Route::Pricing {},
                        class: "hover:text-blue-400 transition",
                        "Pricing"
                    }
                    Link {
                        to: Route::Resources {},
                        class: "hover:text-blue-400 transition",
                        "Resources"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "hover:text-blue-400 transition",
                        "Contact"
                    }
                
                }
            }
        }
    }
}