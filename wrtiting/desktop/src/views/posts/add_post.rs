use dioxus::{logger::tracing, prelude::*};
use server::post_functions::create_post;
use crate::ui::input::{Input, InputType};
use crate::ui::button::{Button, ButtonScheme};

#[component]
pub fn AddPost(
    // Changed to expect the post ID
    on_post_added: EventHandler<i32>,
) -> Element {
    let mut title = use_signal(String::new);
    let mut body = use_signal(String::new);

    let on_submit = move |_| {
        to_owned![title, body];
        spawn(async move {
            let current_title = title.read().clone();
            let current_body = body.read().clone();
            
            // Client-side validation
            if current_title.trim().is_empty() || current_body.trim().is_empty() {
                tracing::warn!("Attempted to submit empty post");
                return;
            }
            
            // Clear fields immediately
            title.set(String::new());
            body.set(String::new());
            
            match create_post(current_title, current_body).await {
                Ok(post_id) => {
                    tracing::info!("Successfully created post with ID: {}", post_id);
                    on_post_added.call(post_id);
                }
                Err(e) => {
                    tracing::error!("Failed to create post: {}", e);
                    // Optionally show error to user
                }
            }
        });
    };

    rsx! {
        div { class: "my-1 flex flex-row items-center gap-2",
            h2 { class: "text-lg font-medium mb-3", "Add New Post" }
            div { class: "space-y-3",
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
                div { class: "mt-4",
                    Button {
                        button_scheme: ButtonScheme::Success,
                        on_click: on_submit,
                        text: "Add Post".to_string(),
                    }
                }
            }
        }
    }
}