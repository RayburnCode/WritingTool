use dioxus::{
    logger::tracing, prelude::*
};
use ui::{Input, InputType};
use ui::{Button, ButtonScheme};
use api::posts::{delete_post, update_post, get_all_posts};

use api::posts::post_model;
use std::sync::Arc;

static POSTS: GlobalSignal<Vec<post_model::Post>> = GlobalSignal::new(Vec::new);



#[component]
pub fn UpdatePost(post: post_model::Post) -> Element {
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
