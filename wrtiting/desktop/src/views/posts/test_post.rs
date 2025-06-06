use dioxus::{logger::tracing, prelude::*};
use shared::models::Post;
use server::post_functions::get_all_posts;


#[derive(Props, Clone, PartialEq)]
pub struct PostsContainerProps {
    #[props(default)]
    pub refresh_count: Signal<u32>,
}

#[component]
pub fn PostsContainer(props: PostsContainerProps) -> Element {
    let mut posts = use_signal(|| Vec::<Post>::new());
    
    // Track the current refresh count
    let mut current_refresh = use_signal(|| ((props.refresh_count)()));

    // Refresh when the count changes
    use_effect(move || {
        if ((props.refresh_count)()) != current_refresh() {
            current_refresh.set((props.refresh_count)());
            
            spawn(async move {
                match get_all_posts().await {
                    Ok(fetched_posts) => posts.set(fetched_posts),
                    Err(err) => tracing::error!("Failed to fetch posts: {err}"),
                }
            });
        }
    });

    rsx! {
        div { class: "mt-6 space-y-4",
            if posts().is_empty() {
                p { class: "text-gray-500", "No posts yet. Add one above!" }
            } else {
                for post in posts().iter() {
                    div { class: "border rounded p-4 bg-white shadow-sm",
                        h2 { class: "text-xl font-semibold", "{post.title}" }
                        p { class: "mt-2", "{post.body}" }
                    }
                }
            }
        }
    }
}