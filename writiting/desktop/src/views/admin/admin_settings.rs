use dioxus::prelude::*;

#[component]
pub fn AdminSettings() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100",
            // Header
            header { class: "bg-white shadow",
                div { class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Admin Settings" }
                    p { class: "mt-1 text-gray-600", "Configure application settings and preferences" }
                }
            }

            // Main Content
            main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
                // Welcome Banner
                div { class: "px-4 pb-6 sm:px-0",
                    div { class: "bg-indigo-50 border-l-4 border-indigo-400 p-4",
                        div { class: "flex items-start",
                            div { class: "flex-shrink-0",
                                svg {
                                    class: "h-5 w-5 text-indigo-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 20 20",
                                    fill: "currentColor",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z",
                                        clip_rule: "evenodd",
                                    }
                                }
                            }
                            div { class: "ml-3",
                                p { class: "text-sm text-indigo-700",
                                    "Welcome to the Admin Settings Page! Make changes carefully as they affect the entire application."
                                }
                            }
                        }
                    }
                }

                // Settings Form
                div { class: "grid grid-cols-1 gap-6 lg:grid-cols-3 px-4 py-6",
                    // Main Settings Card
                    div { class: "bg-white shadow overflow-hidden sm:rounded-lg lg:col-span-2",
                        div { class: "px-4 py-5 sm:p-6",
                            h2 { class: "text-lg font-medium leading-6 text-gray-900",
                                "Application Settings"
                            }
                            form { class: "mt-6 space-y-6",
                                // Site Name Field
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700",
                                        r#for: "site-name",
                                        "Site Name"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            class: "block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            id: "site-name",
                                            name: "site-name",
                                            r#type: "text",
                                            placeholder: "Enter your site name",
                                        }
                                    }
                                }

                                // Admin Email Field
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700",
                                        r#for: "admin-email",
                                        "Admin Email"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            class: "block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            id: "admin-email",
                                            name: "admin-email",
                                            r#type: "email",
                                            placeholder: "admin@example.com",
                                        }
                                    }
                                }

                                // Maintenance Mode Toggle
                                div { class: "flex items-start",
                                    div { class: "flex items-center h-5",
                                        input {
                                            class: "focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded",
                                            id: "maintenance-mode",
                                            name: "maintenance-mode",
                                            r#type: "checkbox",
                                        }
                                    }
                                    div { class: "ml-3 text-sm",
                                        label {
                                            class: "font-medium text-gray-700",
                                            r#for: "maintenance-mode",
                                            "Maintenance Mode"
                                        }
                                        p { class: "text-gray-500",
                                            "When enabled, the site will be unavailable to non-admin users"
                                        }
                                    }
                                }

                                // Save Button
                                div {
                                    button {
                                        r#type: "submit",
                                        class: "inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                                        "Save Settings"
                                    }
                                }
                            }
                        }
                    }

                    // Help and Logs Sidebar
                    div { class: "space-y-6",
                        // Help Card
                        div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                            div { class: "px-4 py-5 sm:p-6",
                                h2 { class: "text-lg font-medium leading-6 text-gray-900",
                                    "Need Help?"
                                }
                                div { class: "mt-2",
                                    p { class: "text-sm text-gray-600",
                                        "If you need assistance with the settings, please contact our support team."
                                    }
                                }
                                div { class: "mt-4",
                                    a {
                                        href: "/help/contact",
                                        class: "inline-flex items-center rounded-md border border-transparent bg-gray-100 px-3 py-2 text-sm font-medium leading-4 text-gray-700 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                                        "Contact Support"
                                    }
                                }
                            }
                        }

                        // Activity Logs Card
                        div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                            div { class: "px-4 py-5 sm:p-6",
                                h2 { class: "text-lg font-medium leading-6 text-gray-900",
                                    "Recent Activity"
                                }
                                div { class: "mt-2",
                                    ul { class: "divide-y divide-gray-200",
                                        li { class: "py-3",
                                            div { class: "flex space-x-3",
                                                svg {
                                                    class: "h-5 w-5 text-gray-400",
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    view_box: "0 0 20 20",
                                                    fill: "currentColor",
                                                    path {
                                                        fill_rule: "evenodd",
                                                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z",
                                                        clip_rule: "evenodd",
                                                    }
                                                }
                                                div { class: "flex-1 space-y-1",
                                                    div { class: "flex items-center justify-between",
                                                        h3 { class: "text-sm font-medium",
                                                            "Changed site name"
                                                        }
                                                        p { class: "text-sm text-gray-500",
                                                            "2 hours ago"
                                                        }
                                                    }
                                                    p { class: "text-sm text-gray-500",
                                                        "Updated to 'My Awesome Site'"
                                                    }
                                                }
                                            }
                                        }
                                        li { class: "py-3",
                                            div { class: "flex space-x-3",
                                                svg {
                                                    class: "h-5 w-5 text-gray-400",
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    view_box: "0 0 20 20",
                                                    fill: "currentColor",
                                                    path { d: "M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" }
                                                    path { d: "M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" }
                                                }
                                                div { class: "flex-1 space-y-1",
                                                    div { class: "flex items-center justify-between",
                                                        h3 { class: "text-sm font-medium",
                                                            "Updated admin email"
                                                        }
                                                        p { class: "text-sm text-gray-500",
                                                            "1 day ago"
                                                        }
                                                    }
                                                    p { class: "text-sm text-gray-500",
                                                        "Changed to admin@example.com"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}