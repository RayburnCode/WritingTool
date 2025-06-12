use dioxus::prelude::*;
use api::posts::find_post;

#[component]
pub fn DisplayPostById(id: i32) -> Element {
    // Use use_resource for async data fetching
    let post = use_resource(move || async move {
        match find_post(id).await {
            Ok(post) => Some(post),
            Err(err) => {
                tracing::error!("Error fetching post {}: {}", id, err);
                None
            }
        }
    });

    rsx! {
        div { class: "my-4",
            match post.read().as_ref() {
                None => rsx! {
                    div { class: "text-center py-8 animate-pulse", "Loading post..." }
                },
                Some(None) => rsx! {
                    div { class: "text-center py-8 text-red-500", "Post not found" }
                },
                Some(Some(post)) => rsx! {
                    div { class: "bg-white rounded-lg shadow-md p-6",
                        h2 { class: "text-2xl font-bold mb-4", "{post.title}" }
                        p { class: "text-gray-700 mb-4", "{post.body}" }
                        div { class: "flex justify-between text-sm text-gray-500",
                            span { {format!("Created: {}", post.created_at.format("%Y-%m-%d %H:%M"))} }
                            span { {format!("Updated: {}", post.updated_at.format("%Y-%m-%d %H:%M"))} }
                        }
                    }
                },
            }
        }
    }
}