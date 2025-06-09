
use dioxus::prelude::*;

#[component]
pub fn AdminSettings() -> Element {
    rsx! {
        p { "Welcome to the Admin Settings Page!" }
        div { class: "admin-settings",
            h1 { "Admin Settings" }
            p { "Here you can configure the application settings." }
            form {
                label { r#for: "site-name", "Site Name:" }
                input {
                    id: "site-name",
                    r#type: "text",
                    placeholder: "Enter site name",
                }

                label { r#for: "admin-email", "Admin Email:" }
                input {
                    id: "admin-email",
                    r#type: "email",
                    placeholder: "Enter admin email",
                }

                label { r#for: "maintenance-mode", "Maintenance Mode:" }
                input { id: "maintenance-mode", r#type: "checkbox" }

                button { r#type: "submit", class: "btn-save", "Save Settings" }
            }
        }
        div { class: "admin-help",
            h2 { "Need Help?" }
            p { "If you need assistance with the settings, please contact support." }
            a { href: "/help/contact", "Contact Support" }
        }
        div { class: "admin-logs",
            h2 { "Activity Logs" }
            p { "Recent changes made to the settings will be logged here." }
            ul {
                li { "Changed site name to 'My Awesome Site'" }
                li { "Updated admin email to " }
            }
        
        }
    }
}