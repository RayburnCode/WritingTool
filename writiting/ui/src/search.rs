use dioxus::prelude::*;

/// Modern search component with clean design and smooth interactions
#[component]
pub fn Search() -> Element {
    let mut search_query = use_signal(|| String::new());

    rsx! {
        div { class: "max-w-md mx-auto",
            form {
                class: "relative",
                onsubmit: move |_| {
                    log::info!("Searching for: {}", search_query());
                },
                // Search input
                input {
                    class: "w-full py-3 px-4 pr-12 rounded-xl border border-gray-200 dark:border-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 shadow-sm",
                    id: "modern-search",
                    placeholder: "Search...",
                    r#type: "text",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
                // Search icon button
                button {
                    class: "absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-blue-500 dark:hover:text-blue-400 transition-colors duration-200",
                    r#type: "submit",
                    aria_label: "Search",
                    svg {
                        class: "w-5 h-5",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" }
                    }
                }
            }
                // Optional: Recent searches dropdown could be added here when focused
        }
    }
}