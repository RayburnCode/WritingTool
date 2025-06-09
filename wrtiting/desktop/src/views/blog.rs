use crate::{views::posts::{AddPost, DisplayPostsById}, Route};
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    // Track the current post ID in state
    let mut current_id = use_signal(|| id.max(0)); // Ensure ID is never negative
    let mut max_post_id = use_signal(|| 0); // Track the highest post ID available

    // Update current_id when the prop changes (from URL navigation)
    use_effect(move || {
        current_id.set(id.max(0));
    });

    rsx! {
        div { class: "max-w-4xl mx-auto p-6",
            // Blog header and navigation
            div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                h1 { class: "text-3xl font-bold text-gray-800 mb-4", "Blog Post #{current_id}" }

                // Navigation controls
                div { class: "flex items-center justify-between mt-6",
                    // Previous button - disabled if at first post
                    if current_id() <= 0 {
                        span { class: "px-4 py-2 rounded transition-colors bg-gray-300 cursor-not-allowed",
                            "Previous"
                        }
                    } else {
                        Link {
                            class: "px-4 py-2 rounded transition-colors bg-blue-500 hover:bg-blue-600 text-white",
                            to: Route::Blog {
                                id: current_id() - 1,
                            },
                            "Previous"
                        }
                    }

                    // Current post indicator
                    div { class: "text-gray-600", "Post {current_id} of {max_post_id}" }

                    // Next button
                    Link {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors",
                        to: Route::Blog {
                            id: current_id() + 1,
                        },
                        "Next"
                    }
                }
            }

            // Display the current post
            DisplayPostsById { id: current_id() }

            // Post creation section
            div { class: "bg-white rounded-lg shadow-md p-6 my-6",
                h2 { class: "text-xl font-bold text-gray-800 mb-4", "Create a New Post" }
                AddPost {
                    on_post_added: move |post_id| {
                        tracing::info!("Post added with ID: {}", post_id);
                        if post_id > max_post_id() {
                            max_post_id.set(post_id);
                        }
                    },
                }
            }
        }
    }
}