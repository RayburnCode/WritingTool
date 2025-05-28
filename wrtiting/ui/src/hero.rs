use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "flex flex-col h-full p-6 bg-white border-b border-gray-200",

            // Top bar with quick actions
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-2xl font-semibold text-gray-800", "My Writing Dashboard" }
                div { class: "flex space-x-3",
                    button {
                        class: "px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors flex items-center gap-2",
                        onclick: move |_| log::info!("New document created"),
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 4v16m8-8H4",
                            }
                        }
                        "New Document"
                    }
                    button {
                        class: "px-4 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors flex items-center gap-2",
                        onclick: move |_| log::info!("Opened folder"),
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M3 7v10a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-6l-2-2H5a2 2 0 0 0-2 2z",
                            }
                        }
                        "Open Project"
                    }
                }
            }

            // Recent documents section
            div { class: "mb-8",
                h2 { class: "text-lg font-medium text-gray-700 mb-4", "Recent Documents" }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    DocumentCard {
                        title: "Novel Draft",
                        last_edited: "2 hours ago",
                        word_count: 1245,
                    }
                    DocumentCard {
                        title: "Research Notes",
                        last_edited: "Yesterday",
                        word_count: 872,
                    }
                    DocumentCard {
                        title: "Blog Post Ideas",
                        last_edited: "3 days ago",
                        word_count: 312,
                    }
                }
            }

            // Writing stats
            div { class: "bg-gray-50 p-4 rounded-lg",
                h2 { class: "text-lg font-medium text-gray-700 mb-3", "Today's Progress" }
                div { class: "flex space-x-6",
                    StatCard {
                        icon: "✍️",
                        label: "Words Written",
                        value: "1,245",
                    }
                    StatCard {
                        icon: "⏱️",
                        label: "Writing Time",
                        value: "2h 18m",
                    }
                    StatCard { icon: "📊", label: "Goal Progress", value: "75%" }
                }
            }
        }
    }
}

#[component]
fn DocumentCard(title: String, last_edited: String, word_count: i32) -> Element {
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
fn StatCard(icon: String, label: String, value: String) -> Element {
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