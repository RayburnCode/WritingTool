use dioxus::prelude::*;
use api::posts::find_post;
use crate::views::posts::UpdatePost;

#[component]
pub fn DisplayPostsById(id: i32) -> Element {
    // Simplified refresh mechanism
    let mut refresh_count = use_signal(|| 0);
    let post = use_resource(move || async move {
        match find_post(id).await {
            Ok(post) => Some(post),
            Err(err) => {
                tracing::error!("Error fetching post: {err}");
                None
            }
        }
    });

    // Provide a simpler refresh function
    let refresh = move || {
        refresh_count += 1;
    };
    provide_context(refresh);

    match post.read().as_ref() {
        Some(Some(post)) => rsx! {
            div {
                UpdatePost { key: "{post.id}", post: post.clone() }
            }
        },
        Some(None) => rsx! {
            div { class: "text-red-500", "Error loading post" }
        },
        None => rsx! {
            div { class: "animate-pulse", "Loading post..." }
        }
    }
}