use dioxus::prelude::*;

#[component]
pub fn SecuritySection(enable_2fa: Signal<bool>) -> Element {
    rsx! {
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
    }
}