use dioxus::prelude::*;

#[component]
pub fn DocumentCard(title: String, last_edited: String, word_count: i32) -> Element {
    rsx! {
        div {
            class: "p-4 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors",
            onclick: move |_| log::info!("Opened document: {}", title),
            h3 { class: "font-medium text-gray-800 truncate", "{title}" }
            div { class: "flex justify-between text-sm text-gray-500 mt-2",
                span { "{last_edited}" }
                span { "{word_count} words" }
            }
        }
    }
}

#[component]
pub fn StatCard(icon: String, label: String, value: String) -> Element {
    rsx! {
        div { class: "flex-1",
            div { class: "flex items-center space-x-3",
                span { class: "text-2xl", icon }
                div {
                    p { class: "text-sm text-gray-500", "{label}" }
                    p { class: "text-xl font-semibold", "{value}" }
                }
            }
        }
    }
}

