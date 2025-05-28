use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-md mt-8",
            id: "blog",

            // Content
            h1 { class: "text-3xl font-bold text-gray-800 mb-4", "This is blog #{id}!" }
            p { class: "text-gray-600 mb-6 leading-relaxed",
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            div { class: "flex items-center justify-center space-x-4",
                Link {
                    class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors",
                    to: Route::Blog { id: id - 1 },
                    "Previous"
                }
                span { class: "text-gray-500", " <---> " }
                Link {
                    class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors",
                    to: Route::Blog { id: id + 1 },
                    "Next"
                }
            }
        }
    }
}