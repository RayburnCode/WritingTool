use dioxus::prelude::*;

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "min-h-[70vh] flex flex-col items-center justify-center bg-gradient-to-br from-gray-50 to-blue-50 p-8 text-center",

            // Main headline
            h1 { class: "text-4xl md:text-5xl font-bold text-gray-800 mb-6",
                "Your Ultimate Rust-Powered Writing Companion"
            }

            // Subtitle
            p { class: "text-xl text-gray-600 max-w-2xl mb-12",
                "Write, organize, and publish with a distraction-free tool built for creators who value performance and privacy."
            }

            // Feature cards grid
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 w-full max-w-6xl",

                // Feature 1
                div { class: "bg-white p-8 rounded-xl shadow-md hover:shadow-lg transition-shadow flex flex-col items-center",
                    h2 { class: "text-2xl font-semibold text-gray-800 mb-4",
                        "📝 Distraction-Free Editor"
                    }
                    p { class: "text-gray-600 mb-4",
                        "Focus mode with Markdown support and live preview."
                    }
                    Link {
                        class: "mt-auto px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors",
                        to: "/editor",
                        "Start Writing →"
                    }
                }

                // Feature 2
                div { class: "bg-white p-8 rounded-xl shadow-md hover:shadow-lg transition-shadow flex flex-col items-center",
                    h2 { class: "text-2xl font-semibold text-gray-800 mb-4",
                        "🗂️ Project Management"
                    }
                    p { class: "text-gray-600 mb-4",
                        "Organize documents with nested folders and tags."
                    }
                    Link {
                        class: "mt-auto px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors",
                        to: "/projects",
                        "Browse Projects →"
                    }
                }

                // Feature 3
                div { class: "bg-white p-8 rounded-xl shadow-md hover:shadow-lg transition-shadow flex flex-col items-center",
                    h2 { class: "text-2xl font-semibold text-gray-800 mb-4",
                        "🔒 Local-First Security"
                    }
                    p { class: "text-gray-600 mb-4",
                        "All data stays on your machine with optional end-to-end encrypted sync."
                    }
                    Link {
                        class: "mt-auto px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors",
                        to: "/settings",
                        "Configure →"
                    }
                }
            }

            // Call-to-action button
            div { class: "mt-16",
                Link {
                    class: "px-8 py-4 bg-indigo-600 text-white text-lg font-semibold rounded-lg hover:bg-indigo-700 transition-colors shadow-lg",
                    to: "/download",
                    "🚀 Download Now"
                }
            }
        }
    }
}