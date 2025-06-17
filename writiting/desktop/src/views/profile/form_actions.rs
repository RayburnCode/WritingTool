use dioxus::prelude::*;


#[component]
pub fn FormActions(is_editing: Signal<bool>) -> Element {
    rsx! {
        div { class: "flex justify-end gap-3",
            if is_editing() {
                button {
                    class: "px-4 py-2 bg-gray-200 rounded hover:bg-gray-300",
                    onclick: move |_| is_editing.set(false),
                    "Cancel"
                }
                button {
                    r#type: "submit",
                    class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                    "Save Changes"
                }
            } else {
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                    onclick: move |_| is_editing.set(true),
                    "Edit Profile"
                }
            }
        }
    }
}