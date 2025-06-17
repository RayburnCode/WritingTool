use dioxus::prelude::*;

#[component]
pub fn NotificationsSection(
    email_notifications: Signal<bool>,
    sms_notifications: Signal<bool>,
) -> Element {
    rsx! {
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
    }
}