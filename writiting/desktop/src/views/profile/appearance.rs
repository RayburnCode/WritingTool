use dioxus::prelude::*;

#[component]
pub fn AppearanceSection(app_theme: Signal<String>) -> Element {
    rsx! {
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