use dioxus::{
    logger::tracing, prelude::*
};
use api::posts::find_post;
use api::posts::post_model;
use std::sync::Arc;
use crate::views::posts::UpdatePost;



#[component]
pub fn DisplayPostsById(id: i32) -> Element {
    // Create a signal to trigger refreshes
    let refresh_counter = Arc::new(std::sync::Mutex::new(0));
    let mut post_signal = use_signal(|| None);
    
    // Single resource with refresh mechanism
    let refresh_counter_clone = Arc::clone(&refresh_counter);
    let resource = {
        let id = id.clone();
        use_resource(move || {
            let refresh_counter = Arc::clone(&refresh_counter_clone);
            let id = id.clone();
            async move {
                // The dependency on refresh_counter causes a refetch when it changes
                let _ = *refresh_counter.lock().unwrap();
                match find_post(id).await {
                    Ok(post) => {
                        post_signal.set(Some(post));
                    }
                    Err(err) => tracing::error!("Error fetching post: {err}"),
                }
            }
        })
    };
    
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
    
    match post_signal().as_ref() {
        Some(post) => {
            rsx! {
                div {
                    UpdatePost { key: "{post.id}", post: post.clone() }
                }
            }
        }
        None => rsx! {
            div { "No Posts Yet" }
        }
    }
}