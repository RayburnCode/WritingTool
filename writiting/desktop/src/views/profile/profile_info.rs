use dioxus::prelude::*;


#[component]
pub fn PersonalInfoSection(
    first_name: Signal<String>,
    last_name: Signal<String>,
    bio: Signal<String>,
    website_url: Signal<String>,
    is_editing: Signal<bool>,
) -> Element {
    rsx! {
        div { class: "mb-8",
            h2 { class: "text-lg font-medium mb-4 text-gray-900", "Personal Information" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                EditableField {
                    label: "First Name".to_string(),
                    value: first_name,
                    is_editing: is_editing(),
                    input_type: "text".to_string(),
                    multiline: false,
                }
                EditableField {
                    label: "Last Name".to_string(),
                    value: last_name,
                    is_editing: is_editing(),
                    input_type: "text".to_string(),
                    multiline: false,
                }
                EditableField {
                    label: "Bio".to_string(),
                    value: bio,
                    is_editing: is_editing(),
                    input_type: "text".to_string(),
                    multiline: true,
                    max_length: Some(2000),
                }
                EditableField {
                    label: "Website URL".to_string(),
                    value: website_url,
                    is_editing: is_editing(),
                    input_type: "url".to_string(),
                    multiline: false,
                    pattern: Some("^https?://".to_string()),
                }
            }
        }
    }
}

#[component]
pub fn EditableField(
    label: String,
    value: Signal<String>,
    is_editing: bool,
    input_type: String,
    multiline: bool,
    #[props(default = None)] max_length: Option<usize>,
    #[props(default = None)] pattern: Option<String>,
) -> Element {
    rsx! {
        div {
            label { class: "block text-sm font-medium text-gray-700 mb-1", "{label}" }
            if is_editing {
                if multiline {
                    textarea {
                        class: "w-full p-2 border rounded",
                        value: "{value}",
                        oninput: move |e| {
                            if let Some(max) = max_length {
                                if e.value().len() <= max {
                                    value.set(e.value())
                                }
                            } else {
                                value.set(e.value())
                            }
                        },
                        maxlength: max_length.map(|m| m.to_string()),
                    }
                } else {
                    input {
                        class: "w-full p-2 border rounded",
                        r#type: "{input_type}",
                        value: "{value}",
                        oninput: move |e| value.set(e.value()),
                        maxlength: max_length.map(|m| m.to_string()),
                        pattern,
                    }
                }
            } else {
                p { class: "p-2 text-gray-900", "{value}" }
            }
            if let Some(max) = max_length {
                div { class: "text-xs text-gray-500 mt-1", "Max {max} characters" }
            }
        }
    }
}