use dioxus::prelude::*;

#[component]
pub fn AccountSettings() -> Element {
    let mut enable_2fa = use_signal(|| false);
    let mut email_notifications = use_signal(|| true);
    let mut sms_notifications = use_signal(|| false);
    let mut app_theme = use_signal(|| "light".to_string());
    
    rsx! {
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