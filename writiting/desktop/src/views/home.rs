
use dioxus::prelude::*;
use ui::{Modal, DocumentCard, StatCard};

#[component]
pub fn Home() -> Element {
        let mut is_modal_open = use_signal(|| false);

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
                        onclick: move |_| is_modal_open.set(true),
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
                        "New Blog Post"
                    }
                }
                // Modal for new blog post
                Modal {
                    is_open: *is_modal_open.read(),
                    on_close: move |_| is_modal_open.set(false),
                    title: "Create New Blog Post".to_string(),
                    div { class: "space-y-4",
                        input {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm",
                            placeholder: "Blog post title",
                            r#type: "text",
                        }
                        textarea {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm",
                            placeholder: "Write your content here...",
                            rows: "5",
                        }
                        button { class: "w-full px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700",
                            "Save Draft"
                        }
                    }
                }
            }
        }

        // Recent documents section
        div { class: "mb-8",
            h2 { class: "text-lg font-medium text-gray-700 mb-4", "Recent Documents" }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                DocumentCard {
                    title: "Getting Started with Gardening",
                    last_edited: "2 hours ago",
                    word_count: 1245,
                }
                DocumentCard {
                    title: "10 Easy Recipes",
                    last_edited: "Yesterday",
                    word_count: 872,
                }
                DocumentCard {
                    title: "Travel Tips for Beginners",
                    last_edited: "3 days ago",
                    word_count: 312,
                }
            }
        }

        // Writing stats
        div { class: "bg-gray-50 p-4 rounded-lg",
            h2 { class: "text-lg font-medium text-gray-700 mb-3", "Today's Progress" }
            div { class: "flex space-x-6",
                StatCard { icon: "✍️", label: "Words Written", value: "1,245" }
                StatCard { icon: "⏱️", label: "Writing Time", value: "2h 18m" }
                StatCard { icon: "📊", label: "Goal Progress", value: "75%" }
            }
        }
    }
    }

