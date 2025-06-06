use dioxus::{
    logger::tracing, prelude::*
};
use crate::ui::input::{Input, InputType};
use crate::ui::button::{Button, ButtonScheme};
use server::post_functions::{delete_post, update_post, get_all_posts};

use shared::models;
use std::sync::Arc;

static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);

#[component]
pub fn Post(post: models::Post) -> Element {
    // Get the refresh function from context
    let refresh_posts = use_context::<Arc<dyn Fn()>>();
    let mut title = use_signal(|| post.title.clone());
    let mut body = use_signal(|| post.body.clone());
    
    // Clone the refresh function outside the closures to avoid move issues
    let update_refresh = refresh_posts.clone();
    let delete_refresh = refresh_posts.clone();
    
    rsx! {
        div { class: "my-1 flex flex-row items-center gap-2",
            Input {
                name: "title".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Title".to_string()),
                value: Some(title()),
                label: Some("Post Title".to_string()),
                oninput: move |event: FormEvent| title.set(event.value()),
            }
            Input {
                name: "body".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Body".to_string()),
                value: Some(body()),
                label: Some("Post Body".to_string()),
                oninput: move |event: FormEvent| body.set(event.value()),
            }
            Button {
                button_scheme: ButtonScheme::Default,
                on_click: move |_| {
                    let title_val = title();
                    let body_val = body();
                    let refresh = update_refresh.clone();
                    spawn(async move {
                        match update_post(post.id, title_val, body_val).await {
                            Ok(_) => {
                                refresh();
                            }
                            Err(err) => tracing::error!("update posts error: {err}"),
                        }
                    });
                },
                text: "Update".to_string(),
            }
            Button {
                button_scheme: ButtonScheme::Danger,
                on_click: move |_| {
                    let refresh = delete_refresh.clone();
                    spawn(async move {
                        match delete_post(post.id).await {
                            Ok(_) => {
                                refresh();
                            }
                            Err(err) => tracing::error!("delete post error: {err}"),
                        }
                    });
                },
                text: "Delete".to_string(),
            }
        }
    }
}

#[component]
pub fn Posts() -> Element {
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
                    Post { key: "{post.id}", post: post.clone() }
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