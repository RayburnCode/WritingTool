use dioxus::prelude::*;
use ui::Avatar;


#[component]
pub fn ProfileHeader(
    avatar_url: Signal<Option<String>>,
    first_name: Signal<String>,
    last_name: Signal<String>,
    is_editing: Signal<bool>,
    is_uploading: Signal<bool>,
) -> Element {
    rsx! {
        div { class: "flex items-center gap-6 mb-8",
            label { class: "relative cursor-pointer group",
                Avatar {
                    class: "w-24 h-24 rounded-full border-2 border-gray-200 group-hover:opacity-90 transition-opacity",
                    src: avatar_url().unwrap_or_default(),
                }
                if is_editing() {
                    div { class: "absolute inset-0 flex items-center justify-center bg-black bg-opacity-40 rounded-full",
                        div { class: "bg-white rounded-full p-2 shadow-md",
                            input {
                                r#type: "file",
                                class: "hidden",
                                id: "avatar-upload",
                                accept: "image/*",
                                onchange: move |evt| {
                                    if let Some(file_engine) = &evt.files() {
                                        if let Some(file) = file_engine.files().get(0) {
                                            is_uploading.set(true);
                                            let file = file.clone();
                                            spawn(async move {
                                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                                is_uploading.set(false);
                                            });
                                        }
                                    }
                                },
                            }
                            label { class: "block", r#for: "avatar-upload",
                                if is_uploading() {
                                    "⏳"
                                } else {
                                    "📷"
                                }
                            }
                        }
                    }
                }
            }
            div {
                h1 { class: "text-2xl font-bold text-gray-900", "Profile Settings" }
                p { class: "text-gray-600", "Manage your personal information" }
            }
        }
    }
}