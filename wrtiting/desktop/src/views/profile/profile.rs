use dioxus::prelude::*;
use ui::Avatar;

#[component]
pub fn Profile() -> Element {
    let mut name = use_signal(|| "John Doe".to_string());
    let mut email = use_signal(|| "john@example.com".to_string());
    let mut phone = use_signal(|| "+1 (555) 123-4567".to_string());
    let mut address = use_signal(|| "123 Main St, Anytown, USA".to_string());
    let mut is_editing = use_signal(|| false);
    let mut profile_pic = use_signal(|| None);
    let mut is_uploading = use_signal(|| false);
    let social_links = use_signal(|| vec![
        ("Twitter", "https://twitter.com", "twitter.svg"),
        ("LinkedIn", "https://linkedin.com", "linkedin.svg"),
        ("GitHub", "https://github.com", "github.svg"),
    ]);

    rsx! {
        div { class: "max-w-4xl mx-auto p-6",
            // Profile header with avatar and upload functionality
            div { class: "flex items-center gap-6 mb-8",
                label { class: "relative cursor-pointer group",
                    Avatar {
                        class: "w-24 h-24 rounded-full border-2 border-gray-200 group-hover:opacity-90 transition-opacity",
                        src: profile_pic().unwrap_or_default(),
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
                                                    profile_pic.set(Some(format!("uploaded/{}", file)));
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

            // Editable profile form
            div { class: "bg-white rounded-lg shadow p-6 mb-6",
                form { onsubmit: move |_| is_editing.set(false),
                    // Personal Information Section
                    div { class: "mb-8",
                        h2 { class: "text-lg font-medium mb-4 text-gray-900", "Personal Information" }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            // Name field
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Full Name"
                                }
                                if is_editing() {
                                    input {
                                        class: "w-full p-2 border rounded",
                                        value: "{name}",
                                        oninput: move |e| name.set(e.value()),
                                    }
                                } else {
                                    p { class: "p-2 text-gray-900", "{name}" }
                                }
                            }

                            // Email field
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Email"
                                }
                                if is_editing() {
                                    input {
                                        class: "w-full p-2 border rounded",
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value()),
                                    }
                                } else {
                                    p { class: "p-2 text-gray-900", "{email}" }
                                }
                            }

                            // Phone field
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Phone"
                                }
                                if is_editing() {
                                    input {
                                        class: "w-full p-2 border rounded",
                                        value: "{phone}",
                                        oninput: move |e| phone.set(e.value()),
                                    }
                                } else {
                                    p { class: "p-2 text-gray-900", "{phone}" }
                                }
                            }

                            // Address field
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Address"
                                }
                                if is_editing() {
                                    textarea {
                                        class: "w-full p-2 border rounded",
                                        value: "{address}",
                                        oninput: move |e| address.set(e.value()),
                                    }
                                } else {
                                    p { class: "p-2 text-gray-900", "{address}" }
                                }
                            }
                        }
                    }

                    // Action buttons
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

            // Social Media Links Section
            div { class: "bg-white rounded-lg shadow p-6",
                h2 { class: "text-lg font-medium mb-4 text-gray-900", "Social Profiles" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                    for (platform , url , icon) in social_links().iter() {
                        div { class: "flex items-center gap-3 p-3 hover:bg-gray-50 rounded transition-colors",
                            img {
                                class: "w-5 h-5",
                                src: "/icons/{icon}",
                                alt: "{platform}",
                            }
                            a {
                                href: "{url}",
                                target: "_blank",
                                class: "text-gray-700 hover:text-blue-600",
                                "{platform}"
                            }
                        }
                    }
                }
            }
        }
    }
}