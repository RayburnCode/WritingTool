use dioxus::{logger::tracing, prelude::*};
use api::posts::create_post;
use ui::{Input, InputType};
use ui::{Button, ButtonScheme};

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
        div { class: "max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-md",
            h2 { class: "text-2xl  font-bold text-gray-800 mb-6", "Add New Blog Post" }
            div { class: "space-y-6 flex flex-row",
                Input {
                    name: "title".to_string(),
                    input_type: Some(InputType::Text),
                    placeholder: Some("Enter post title...".to_string()),
                    value: Some(title()),
                    label: Some("Post Title".to_string()),
                    oninput: move |event: FormEvent| title.set(event.value()),
                }
                Input {
                    name: "body".to_string(),
                    input_type: Some(InputType::Text),
                    placeholder: Some("Write your post content here...".to_string()),
                    value: Some(body()),
                    label: Some("Post Content".to_string()),
                    oninput: move |event: FormEvent| body.set(event.value()),
                }
                div { class: "flex justify-end mt-6",
                    Button {
                        button_scheme: ButtonScheme::Success,
                        class: "px-6 py-2",
                        on_click: on_submit,
                        text: "Publish Post".to_string(),
                    }
                }
            }
        }
    }

   
}