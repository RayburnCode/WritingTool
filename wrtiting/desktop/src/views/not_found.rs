use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    // Format path outside of rsx! macro
    let path_display = if !route.is_empty() {
        format!("/{}", route.join("/"))
    } else {
        "".to_string()
    };

    rsx! {
        div { class: "container mx-auto px-4 py-8 text-center",
            h1 { class: "text-3xl font-bold text-red-600 mb-4", "404 - Page Not Found" }
            p { class: "text-lg mb-4",
                "Sorry, the page "
                span { "{path_display}" }
                " does not exist."
            }
            a { class: "text-blue-500 hover:underline", href: "/", "Return to home page" }
        }
    }
}