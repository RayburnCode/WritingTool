use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    User(String),
    AI(String),
    Error(String),
}

#[derive(Default)]
pub struct ChatState {
    messages: Vec<Message>,
    current_prompt: String,
    is_loading: bool,
}

#[component]
pub fn Chat() -> Element {
    let mut state = use_signal(ChatState::default);
    let textarea_ref = use_signal::<Option<Element>>(|| None);

    // Handle prompt input
    let mut set_prompt = move |value: String| {
        state.with_mut(|s| s.current_prompt = value);
    };

    // Send prompt to AI
    let send_prompt = {
        let state = state.clone();
        move || {
            let mut state = state.clone();
            spawn(async move {
                let prompt = state.with(|s| s.current_prompt.clone());
                if prompt.trim().is_empty() {
                    return;
                }

                // Add user message
                state.with_mut(|s| {
                    s.messages.push(Message::User(prompt.clone()));
                    s.current_prompt.clear();
                    s.is_loading = true;
                });

                // Simulate AI response (replace with actual API call)
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                
                state.with_mut(|s| {
                    s.messages.push(Message::AI(format!("Response to: {}", prompt)));
                    s.is_loading = false;
                });
            });
        }
    };

// Handle Enter key press
let handle_keydown = move |e: KeyboardEvent| {
    if e.key() == Key::Enter && !e.modifiers().shift() {
        e.prevent_default();
        send_prompt();
    }
};


    rsx! {
        div { class: "flex flex-col h-screen max-w-3xl mx-auto p-4 bg-gray-50",
            h2 { class: "text-2xl font-bold mb-4 text-gray-800", "Chat with AI" }
            // Message history
            div {
                class: "flex-1 overflow-y-auto mb-4 space-y-4",
                id: "chat-history",
                {
                    state
                        .read()
                        .messages
                        .iter()
                        .map(|msg| {
                            match msg {
                                Message::User(text) => rsx! {
                                    div { class: "flex justify-end",
                                        div { class: "bg-blue-500 text-white rounded-lg py-2 px-4 max-w-xs md:max-w-md lg:max-w-lg",
                                            "{text}"
                                        }
                                    }
                                },
                                Message::AI(text) => rsx! {
                                    div { class: "flex justify-start",
                                        div { class: "bg-gray-200 text-gray-800 rounded-lg py-2 px-4 max-w-xs md:max-w-md lg:max-w-lg",
                                            "{text}"
                                        }
                                    }
                                },
                                Message::Error(err) => rsx! {
                                    div { class: "flex justify-center",
                                        div { class: "bg-red-100 text-red-800 rounded-lg py-2 px-4 max-w-md", "⚠️ {err}" }
                                    }
                                },
                            }
                        })
                }

                {
                    if state.read().is_loading {
                        Some(rsx! {
                            div { class: "flex justify-start",
                                div { class: "bg-gray-200 text-gray-800 rounded-lg py-2 px-4 max-w-xs animate-pulse",
                                    "Thinking..."
                                }
                            }
                        })
                    } else {
                        None
                    }
                }
            }
            // Input area
            div { class: "flex flex-col gap-2",
                textarea {
                    class: "w-full p-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                    rows: "3",
                    value: "{state.read().current_prompt}",
                    oninput: move |e| set_prompt(e.value()),
                    onkeydown: handle_keydown,
                    placeholder: "Type your message here...",
                    disabled: state.read().is_loading,
                }
                button {
                    class: "self-end bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 px-4 rounded-lg disabled:opacity-50",
                    onclick: move |_| send_prompt(),
                    disabled: state.read().current_prompt.trim().is_empty() || state.read().is_loading,
                    if state.read().is_loading {
                        "Sending..."
                    } else {
                        "Send"
                    }
                }
            }
        }
    }

}

