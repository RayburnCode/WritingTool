use dioxus::{
    logger::tracing, prelude::*
};

use api::posts::get_all_posts;
use api::posts::post_model;
use std::sync::Arc;

use crate::views::posts::UpdatePost;


static POSTS: GlobalSignal<Vec<post_model::Post>> = GlobalSignal::new(Vec::new);


#[component]
pub fn DisplayAllPosts() -> Element {
    // Create a signal to trigger refreshes
    let refresh_counter = Arc::new(std::sync::Mutex::new(0));
    
    // Single resource with refresh mechanism
    let refresh_counter_clone = Arc::clone(&refresh_counter);
    let resource = use_resource(move || {
        let refresh_counter = Arc::clone(&refresh_counter_clone);
        async move {
            // The dependency on refresh_counter causes a refetch when it changes
            let _ = *refresh_counter.lock().unwrap();
            match get_all_posts().await {
                Ok(posts) => *POSTS.write() = posts,
                Err(err) => tracing::error!("get all post error: {err}"),
            }
        }
    });
    
    // Function to trigger a refresh
    let refresh_posts = {
        let refresh_counter = Arc::clone(&refresh_counter);
        move || {
            let mut counter = refresh_counter.lock().unwrap();
            *counter += 1;
        }
    };
    
    // Make refresh_posts available to other components
    provide_context(Arc::new(refresh_posts) as Arc<dyn Fn()>);
    
    match resource() {
        Some(_) if !POSTS().is_empty() => {
            let posts_data = POSTS();
            let posts = posts_data.iter().map(|post| {
                rsx! {
                    UpdatePost { key: "{post.id}", post: post.clone() }
                }
            });
            rsx! {
                div { {posts} }
            }
        }
        _ => rsx! {
            div { "No Posts Yet" }
        }
    }
}




