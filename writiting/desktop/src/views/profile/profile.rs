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
    let mut enable_2fa = use_signal(|| false);
    let mut email_notifications = use_signal(|| true);
    let mut sms_notifications = use_signal(|| false);
    let mut app_theme: Signal<String> = use_signal(|| "light".to_string());
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
        div { class: "max-w-4xl mx-auto p-6",
            // Header
            div { class: "mb-8",
                h1 { class: "text-2xl font-bold text-gray-900", "Account Settings" }
                p { class: "text-gray-600", "Manage your security and application preferences" }
            }

            // Security Section
            div { class: "bg-white rounded-lg shadow p-6 mb-6",
                h2 { class: "text-lg font-medium mb-4 text-gray-900", "Security" }
                // Password Change
                div { class: "mb-4 p-4 border-b",
                    h3 { class: "font-medium text-gray-800 mb-2", "Password" }
                    div { class: "flex justify-between items-center",
                        p { class: "text-gray-600", "Last changed 3 months ago" }
                        button { class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                            "Change Password"
                        }
                    }
                }

                // 2FA
                div { class: "p-4",
                    h3 { class: "font-medium text-gray-800 mb-2", "Two-Factor Authentication" }
                    div { class: "flex justify-between items-center",
                        p { class: "text-gray-600",
                            if enable_2fa() {
                                "Enabled (SMS verification)"
                            } else {
                                "Disabled - adds extra security"
                            }
                        }
                        button {
                            class: "px-4 py-2 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| enable_2fa.set(!enable_2fa()),
                            if enable_2fa() {
                                "Disable"
                            } else {
                                "Enable"
                            }
                        }
                    }
                }
            }

            // Notifications Section
            div { class: "bg-white rounded-lg shadow p-6 mb-6",
                h2 { class: "text-lg font-medium mb-4 text-gray-900", "Notifications" }
                div { class: "space-y-4",
                    // Email Notifications
                    div { class: "flex justify-between items-center p-3 hover:bg-gray-50 rounded",
                        div {
                            h3 { class: "font-medium text-gray-800", "Email Notifications" }
                            p { class: "text-sm text-gray-600", "Receive updates via email" }
                        }
                        input {
                            r#type: "checkbox",
                            class: "h-5 w-5 text-blue-600 rounded",
                            checked: email_notifications(),
                            onchange: move |e| email_notifications.set(e.value().parse().unwrap_or(false)),
                        }
                    }

                    // SMS Notifications
                    div { class: "flex justify-between items-center p-3 hover:bg-gray-50 rounded",
                        div {
                            h3 { class: "font-medium text-gray-800", "SMS Notifications" }
                            p { class: "text-sm text-gray-600", "Receive text message alerts" }
                        }
                        input {
                            r#type: "checkbox",
                            class: "h-5 w-5 text-blue-600 rounded",
                            checked: sms_notifications(),
                            onchange: move |e| sms_notifications.set(e.value().parse().unwrap_or(false)),
                        }
                    }
                }
            }

            // Connected Apps
            div { class: "bg-white rounded-lg shadow p-6 mb-6",
                h2 { class: "text-lg font-medium mb-4 text-gray-900", "Connected Applications" }
                div { class: "space-y-3",
                    // Google
                    div { class: "flex items-center justify-between p-3 bg-gray-50 rounded",
                        div { class: "flex items-center gap-3",
                            img {
                                class: "w-6 h-6",
                                src: "/icons/google.svg",
                                alt: "Google",
                            }
                            span { "Google" }
                        }
                        button { class: "text-blue-600 hover:underline", "Disconnect" }
                    }

                    // Other service
                    div { class: "flex items-center justify-between p-3 bg-gray-50 rounded",
                        div { class: "flex items-center gap-3",
                            img {
                                class: "w-6 h-6",
                                src: "/icons/ghl.svg",
                                alt: "GHL",
                            }
                            span { "GoHighLevel" }
                        }
                        button { class: "text-blue-600 hover:underline", "Disconnect" }
                    }
                }
            }

            // Appearance
            div { class: "bg-white rounded-lg shadow p-6",
                h2 { class: "text-lg font-medium mb-4 text-gray-900", "Appearance" }
                div { class: "flex gap-4",
                    button {
                        class: {
                            let base = "px-4 py-2 rounded border-2 ";
                            if app_theme() == "light" {
                                format!("{}border-blue-500 bg-blue-50", base)
                            } else {
                                format!("{}border-transparent", base)
                            }
                        },
                        onclick: move |_| app_theme.set("light".to_string()),
                        "Light"
                    }
                    button {
                        class: {
                            let base = "px-4 py-2 rounded border-2 ";
                            if app_theme() == "dark" {
                                format!("{}border-blue-500 bg-blue-50", base)
                            } else {
                                format!("{}border-transparent", base)
                            }
                        },
                        onclick: move |_| app_theme.set("dark".to_string()),
                        "Dark"
                    }
                    button {
                        class: {
                            let base = "px-4 py-2 rounded border-2 ";
                            if app_theme() == "system" {
                                format!("{}border-blue-500 bg-blue-50", base)
                            } else {
                                format!("{}border-transparent", base)
                            }
                        },
                        onclick: move |_| app_theme.set("system".to_string()),
                        "System"
                    }
                }
            }
        }
    }
}