use crate::{views::posts::{AddPost, DisplayPostsById}, Route};
use dioxus::prelude::*;
use api::posts::get_max_post_id;

#[component]
pub fn Blog(id: i32) -> Element {
    // Track the current post ID and max ID
    let mut current_id = use_signal(|| id.max(1));
    let mut max_post_id = use_signal(|| None);
    
    // Fetch the actual max post ID from the server
    let posts_count = use_resource(move || async move {
        match get_max_post_id().await {
            Ok(count) => Some(count),
            Err(err) => {
                tracing::error!("Failed to fetch post count: {}", err);
                None
            }
        }
    }); 

    // Update current_id when the prop changes
    use_effect(move || {
        current_id.set(id.max(1));
    });

    // Update max_post_id when posts_count changes
    use_effect(move || {
        if let Some(Some(count)) = posts_count.read().as_ref() {
            max_post_id.set(Some(*count));
        }
    });

    // Navigation handlers
    let navigate_prev = move |_| {
        if current_id() > 1 {
            current_id.set(current_id() - 1);
        }
    };

    let navigate_next = move |_| {
        if let Some(max) = max_post_id() {
            if current_id() < max {
                current_id.set(current_id() + 1);
            }
        }
    };

    rsx! {
        div { class: "max-w-4xl mx-auto p-6",
            // Blog header and navigation
            div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                h1 { class: "text-3xl font-bold text-gray-800 mb-4", 
                    "Blog Post #{current_id}" 
                }

                // Navigation controls
                div { class: "flex items-center justify-between mt-6",
                    // Previous button
                    button {
                        class: "px-4 py-2 rounded transition-colors {if current_id() <= 1 { 
                            \"bg-gray-300 cursor-not-allowed\" 
                        } else { 
                            \"bg-blue-500 hover:bg-blue-600 text-white\" 
                        }}",
                        disabled: current_id() <= 1,
                        onclick: navigate_prev,
                        "Previous"
                    }

                    // Current post indicator
                    div { class: "text-gray-600 flex items-center gap-2",
                        "Post {current_id}"
                        if let Some(max) = max_post_id() {
                            rsx! { "of {max}" }
                        } else {
                            rsx! { span { class: "animate-pulse", "Loading..." } }
                        }
                    }

                    // Next button
                    button {
                        class: "px-4 py-2 rounded transition-colors {if max_post_id().is_none() || current_id() >= max_post_id().unwrap_or(0) { 
                            \"bg-gray-300 cursor-not-allowed\" 
                        } else { 
                            \"bg-blue-500 hover:bg-blue-600 text-white\" 
                        }}",
                        disabled: max_post_id().is_none() || current_id() >= max_post_id().unwrap_or(0),
                        onclick: navigate_next,
                        "Next"
                    }
                }
            }

            // Display the current post with loading state
            match max_post_id() {
                Some(max) if current_id() <= max => rsx! {
                    DisplayPostsById { id: current_id() }
                },
                Some(_) => rsx! {
                    div { class: "text-center py-8 text-red-500",
                        "Post not found"
                    }
                },
                None => rsx! {
                    div { class: "text-center py-8 animate-pulse",
                        "Loading post..."
                    }
                }
            }

            // Post creation section
            div { class: "bg-white rounded-lg shadow-md p-6 my-6",
                h2 { class: "text-xl font-bold text-gray-800 mb-4", 
                    "Create a New Post" 
                }
                AddPost {
                    on_post_added: move |post_id| {
                        tracing::info!("Post added with ID: {}", post_id);
                        // Update the max ID if this is a new highest ID
                        if max_post_id().map_or(true, |max| post_id > max) {
                            max_post_id.set(Some(post_id));
                        }
                        // Navigate to the new post
                        current_id.set(post_id);
                    },
                }
            }
        }
    }
}

