use crate::views::posts::{AddPost, DisplayPostById};
use dioxus::prelude::*;
use api::posts::get_post_count;

#[component]
pub fn Blog(initial_id: i32) -> Element {
    // Track the current post ID and max ID
    let mut current_id: Signal<i32> = use_signal(|| initial_id.max(1));
    let mut max_post_id: Signal<Option<i32>> = use_signal(|| None);
    
    // Fetch the actual max post ID from the server
    let posts_count = use_resource(move || async move {
        match get_post_count().await {
            Ok(count) => Some(count),
            Err(err) => {
                tracing::error!("Failed to fetch post count: {}", err);
                None
            }
        }
    });

    // Update current_id when the prop changes
    use_effect(move || {
        current_id.set(initial_id.max(1));
    });

    // Update max_post_id when posts_count changes
    use_effect(move || {
        if let Some(Some(count)) = posts_count.read().as_ref() {
            max_post_id.set((*count).try_into().ok());
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
                    {format!("Blog Post #{}", current_id())}
                }
                // Navigation controls
                div { class: "flex items-center justify-between mt-6",
                    // Previous button
                    button {
                        class: if current_id() <= 1 { "px-4 py-2 rounded transition-colors bg-gray-300 cursor-not-allowed" } else { "px-4 py-2 rounded transition-colors bg-blue-500 hover:bg-blue-600 text-white" },
                        disabled: current_id() <= 1,
                        onclick: navigate_prev,
                        "Previous"
                    }

                    // Current post indicator
                    div { class: "text-gray-600 flex items-center gap-2",
                        {format!("Post {}", current_id())}
                        {
                            max_post_id()
                                .map(|max| rsx! {
                                    span { {format!(" of {}", max)} }
                                })
                                .unwrap_or(rsx! {
                                    span { " " }
                                })
                        }
                        {
                            if max_post_id().is_none() {
                                rsx! {
                                    span { class: "animate-pulse", "Loading..." }
                                }
                            } else {
                                rsx! {}
                            }
                        }
                    }

                    // Next button
                    button {
                        class: if max_post_id().is_none() || current_id() >= max_post_id().unwrap_or(0) { "px-4 py-2 rounded transition-colors bg-gray-300 cursor-not-allowed" } else { "px-4 py-2 rounded transition-colors bg-blue-500 hover:bg-blue-600 text-white" },
                        disabled: max_post_id().is_none() || current_id() >= max_post_id().unwrap_or(0),
                        onclick: navigate_next,
                        "Next"
                    }
                }
            }

            // Post display section
            div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                match max_post_id() {
                    Some(max) if current_id() <= max => rsx! {
                        DisplayPostById { id: current_id() }
                    },
                    Some(_) => rsx! {
                        div { class: "text-center py-8 text-red-500", "Post not found" }
                    },
                    None => rsx! {
                        div { class: "text-center py-8 animate-pulse", "Loading post..." }
                    },
                }
            }

            // Add new post section
            div { class: "bg-white rounded-lg shadow-md p-6",
                h2 { class: "text-xl font-bold text-gray-800 mb-4", "Create a New Post" }
                AddPost {
                    on_post_added: move |new_post_id| {
                        tracing::info!("Post added with ID: {}", new_post_id);
                        if max_post_id().map_or(true, |max| new_post_id > max) {
                            max_post_id.set(Some(new_post_id));
                        }
                        current_id.set(new_post_id);
                    },
                }
            }
        }
    }
}