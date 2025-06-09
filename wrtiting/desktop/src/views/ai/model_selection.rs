use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AIModel {
    Ollama,
    OpenAI,
    Anthropic,
}

impl Default for AIModel {
    fn default() -> Self {
        AIModel::Ollama
    }
}

impl From<&str> for AIModel {
    fn from(value: &str) -> Self {
        match value {
            "openai" => AIModel::OpenAI,
            "anthropic" => AIModel::Anthropic,
            _ => AIModel::Ollama,
        }
    }
}

#[derive(Default)]
pub struct ModelState {
    selected_model: AIModel,
    api_key: String,
    is_connected: bool,
    connection_error: Option<String>,
}

#[component]
pub fn ModelSelection(on_connect: EventHandler<(AIModel, String)>) -> Element {
    let mut state = use_signal(ModelState::default);

    // Update selected model
    let set_model = move |value: String| {
        state.with_mut(|s| {
            s.selected_model = value.as_str().into();
            // Clear API key when switching models (for security)
            s.api_key.clear();
            s.is_connected = false;
            s.connection_error = None;
        });
    };

    // Update API key
    let set_api_key = move |value: String| {
        state.with_mut(|s| {
            s.api_key = value;
            s.is_connected = false;
            s.connection_error = None;
        });
    };

    // Connect to AI service
    let connect_to_ai = move || {
        let model = state.with(|s| s.selected_model.clone());
        let api_key = state.with(|s| s.api_key.clone());
        
        state.with_mut(|s| {
            s.is_connected = false;
            s.connection_error = None;
        });

        // Validate input (example validation)
        if matches!(model, AIModel::OpenAI | AIModel::Anthropic) && api_key.trim().is_empty() {
            state.with_mut(|s| {
                s.connection_error = Some("API key is required for this model".to_string());
            });
            return;
        }

        // In a real app, you would make an actual connection here
        // For now, we'll simulate a successful connection
        state.with_mut(|s| {
            s.is_connected = true;
        });

        // Notify parent component
        on_connect.call((model, api_key));
    };

    rsx! {
        div { class: "flex flex-col gap-4 p-4 border rounded-lg",
            h2 { class: "text-xl font-bold", "Select AI Model" }
            
            select {
                class: "p-2 border rounded",
                disabled: state.read().is_connected,
                onchange: move |e| set_model(e.value()),
                option { value: "ollama", "Ollama (Local)" }
                option { value: "openai", "OpenAI" }
                option { value: "anthropic", "Anthropic" }
            }

            // Show API key input only for cloud-based models
            {match state.read().selected_model {
                AIModel::Ollama => rsx! { 
                    div { class: "text-sm text-gray-500", "No API key needed for local Ollama" } 
                },
                _ => rsx! {
                    input {
                        class: "p-2 border rounded",
                        r#type: "password",
                        placeholder: "API Key (required)",
                        value: "{state.read().api_key}",
                        disabled: state.read().is_connected,
                        oninput: move |e| set_api_key(e.value()),
                    }
                }
            }}

            // Connection status and error messages
            div {
                if let Some(err) = &state.read().connection_error {
                    rsx! { div { class: "text-red-500 text-sm", "Error: {err}" } }
                } else if state.read().is_connected {
                    rsx! { div { class: "text-green-500 text-sm", "Connected successfully!" } }
                }
            }

            button {
                class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50",
                disabled: state.read().is_connected || 
                    (matches!(state.read().selected_model, AIModel::OpenAI | AIModel::Anthropic) 
                    && state.read().api_key.trim().is_empty(),
                onclick: move |_| connect_to_ai(),
                if state.read().is_connected {
                    "Connected"
                } else {
                    "Connect"
                }
            }
        }
    }
}