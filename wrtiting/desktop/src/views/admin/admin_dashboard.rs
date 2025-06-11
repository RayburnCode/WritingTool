use dioxus::prelude::*;

#[component]
pub fn AdminDashboard() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100",
            // Header
            header { class: "bg-white shadow",
                div { class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Admin Dashboard" }
                }
            }

            // Main Content
            main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
                // Welcome Section
                div { class: "px-4 py-6 sm:px-0",
                    div { class: "border-4 border-dashed border-gray-200 rounded-lg p-6",
                        h2 { class: "text-xl font-semibold text-gray-800",
                            "Welcome to the Admin Dashboard!"
                        }
                        p { class: "mt-2 text-gray-600",
                            "Here you can manage users, view reports, and perform administrative tasks."
                        }
                    }
                }

                // Stats Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 px-4 py-6",
                    // Users Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Total Users" }
                            p { class: "mt-1 text-3xl font-semibold text-indigo-600",
                                "1,024"
                            }
                        }
                    }
                    // Reports Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Reported Content" }
                            p { class: "mt-1 text-3xl font-semibold text-red-600",
                                "50"
                            }
                        }
                    }
                    // Sessions Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Active Sessions" }
                            p { class: "mt-1 text-3xl font-semibold text-green-600",
                                "200"
                            }
                        }
                    }
                }

                // Action Buttons
                div { class: "px-4 py-6 grid grid-cols-1 gap-6 sm:grid-cols-3",
                    a {
                        href: "/admin/users",
                        class: "bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-3 px-4 rounded-lg text-center transition duration-150 ease-in-out",
                        "Manage Users"
                    }
                    a {
                        href: "/admin/reportes",
                        class: "bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-4 rounded-lg text-center transition duration-150 ease-in-out",
                        "View Reports"
                    }
                    a {
                        href: "/admin/settings",
                        class: "bg-gray-600 hover:bg-gray-700 text-white font-bold py-3 px-4 rounded-lg text-center transition duration-150 ease-in-out",
                        "Settings"
                    }
                }
            }
        }
    }
}