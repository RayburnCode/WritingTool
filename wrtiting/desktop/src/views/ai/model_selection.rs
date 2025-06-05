use dioxus::prelude::*;

#[component]
pub fn ModelSelection() -> Element {
rsx! {
    div {
        h2 { "Select AI Model" }
        select { onchange: move |e| set_model(e.value),
            option { value: "ollama", "Ollama (Local)" }
            option { value: "openai", "OpenAI" }
            option { value: "anthropic", "Anthropic" }
        }
        input {
            r#type: "password",
            placeholder: "API Key (if needed)",
            oninput: move |e| set_api_key(e.value),
        }
        button { onclick: connect_to_ai, "Connect" }
    }
}
}




