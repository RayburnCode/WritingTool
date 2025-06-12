use dioxus::prelude::*;
use api::posts::find_post;

#[component]
pub fn DisplayPostById(id: i32) -> Element {
    // State to hold the post data and loading/error states
    let mut post = use_signal(|| None);
    let mut is_loading = use_signal(|| true);
    let mut error = use_signal(|| None);

    // Fetch the post when the component mounts or when the ID changes
    use_effect(move || {
        spawn(async move {
            is_loading.set(true);
            error.set(None);
            
            match find_post(id).await {
                Ok(fetched_post) => {
                    post.set(Some(fetched_post));
                    error.set(None);
                }
                Err(err) => {
                    tracing::error!("Error fetching post: {err}");
                    error.set(Some(format!("Failed to load post: {err}")));
                    post.set(None);
                }
            }
            
            is_loading.set(false);
        });
    });

    rsx! {
        div { class: "my-4",
            if is_loading() {
                div { class: "text-center py-8 animate-pulse", "Loading post..." }
            } else if let Some(err) = error() {
                div { class: "text-center py-8 text-red-500", "{err}" }
            } else if let Some(post) = post() {
                div { class: "bg-white rounded-lg shadow-md p-6",
                    h2 { class: "text-2xl font-bold mb-4", "{post.title}" }
                    p { class: "text-gray-700 mb-4", "{post.body}" }
                    div { class: "flex justify-between text-sm text-gray-500",
                        span { {format_args!("Created: {}", post.created_at.format("%Y-%m-%d %H:%M"))} }
                        span { {format_args!("Updated: {}", post.updated_at.format("%Y-%m-%d %H:%M"))} }
                    }
                }
            } else {
                div { class: "text-center py-8 text-red-500", "Post not found" }
            }
        }
    }
}