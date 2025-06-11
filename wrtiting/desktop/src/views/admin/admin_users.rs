use dioxus::prelude::*;

#[component]
pub fn AdminUsers() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100",
            // Header
            header { class: "bg-white shadow",
                div { class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "User Management" }
                    p { class: "mt-1 text-gray-600",
                        "Manage users, view profiles, and perform administrative tasks"
                    }
                }
            }

            // Main Content
            main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
                // Welcome Banner
                div { class: "px-4 pb-6 sm:px-0",
                    div { class: "bg-indigo-50 border-l-4 border-indigo-400 p-4 rounded-lg",
                        div { class: "flex items-center",
                            svg {
                                class: "h-5 w-5 text-indigo-400 mr-3",
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                path { d: "M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v1h8v-1zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-1a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v1h-3zM4.75 12.094A5.973 5.973 0 004 15v1H1v-1a3 3 0 013.75-2.906z" }
                            }
                            p { class: "text-sm text-indigo-700",
                                "Welcome to the Admin Users Page! Manage your user base efficiently."
                            }
                        }
                    }
                }

                // Stats Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 px-4 py-6",
                    // Total Users Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Total Users" }
                            p { class: "mt-1 text-3xl font-semibold text-indigo-600",
                                "1,000"
                            }
                        }
                    }
                    // Active Users Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Active Users" }
                            p { class: "mt-1 text-3xl font-semibold text-green-600",
                                "800"
                            }
                        }
                    }
                    // Inactive Users Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Inactive Users" }
                            p { class: "mt-1 text-3xl font-semibold text-gray-600",
                                "200"
                            }
                        }
                    }
                }

                // User Table
                div { class: "px-4 py-6",
                    div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                        div { class: "px-4 py-5 sm:px-6 border-b border-gray-200",
                            h2 { class: "text-lg font-medium leading-6 text-gray-900",
                                "User List"
                            }
                        }
                        div { class: "overflow-x-auto",
                            table { class: "min-w-full divide-y divide-gray-200",
                                thead { class: "bg-gray-50",
                                    tr {
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "Name"
                                        }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "Email"
                                        }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "Status"
                                        }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "Actions"
                                        }
                                    }
                                }
                                tbody { class: "bg-white divide-y divide-gray-200",
                                    // User 1
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap",
                                            div { class: "flex items-center",
                                                div { class: "flex-shrink-0 h-10 w-10",
                                                    img {
                                                        class: "h-10 w-10 rounded-full",
                                                        src: "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=4&w=256&h=256&q=60",
                                                        alt: "",
                                                    }
                                                }
                                                div { class: "ml-4",
                                                    div { class: "text-sm font-medium text-gray-900",
                                                        "John Doe"
                                                    }
                                                    div { class: "text-sm text-gray-500",
                                                        "Admin"
                                                    }
                                                }
                                            }
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                            "john@example.com"
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap",
                                            span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
                                                "Active"
                                            }
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                            div { class: "flex space-x-2",
                                                button { class: "text-indigo-600 hover:text-indigo-900",
                                                    "Edit"
                                                }
                                                button { class: "text-red-600 hover:text-red-900",
                                                    "Delete"
                                                }
                                                button { class: "text-gray-600 hover:text-gray-900",
                                                    "View"
                                                }
                                            }
                                        }
                                    }
                                    // User 2
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap",
                                            div { class: "flex items-center",
                                                div { class: "flex-shrink-0 h-10 w-10",
                                                    img {
                                                        class: "h-10 w-10 rounded-full",
                                                        src: "https://images.unsplash.com/photo-1573496359142-b8d87734a5a2?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=4&w=256&h=256&q=60",
                                                        alt: "",
                                                    }
                                                }
                                                div { class: "ml-4",
                                                    div { class: "text-sm font-medium text-gray-900",
                                                        "Jane Smith"
                                                    }
                                                    div { class: "text-sm text-gray-500",
                                                        "Editor"
                                                    }
                                                }
                                            }
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                            "jane@example.com"
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap",
                                            span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
                                                "Active"
                                            }
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                            div { class: "flex space-x-2",
                                                button { class: "text-indigo-600 hover:text-indigo-900",
                                                    "Edit"
                                                }
                                                button { class: "text-red-600 hover:text-red-900",
                                                    "Delete"
                                                }
                                                button { class: "text-gray-600 hover:text-gray-900",
                                                    "View"
                                                }
                                            }
                                        }
                                    }
                                    // User 3
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap",
                                            div { class: "flex items-center",
                                                div { class: "flex-shrink-0 h-10 w-10",
                                                    img {
                                                        class: "h-10 w-10 rounded-full",
                                                        src: "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=4&w=256&h=256&q=60",
                                                        alt: "",
                                                    }
                                                }
                                                div { class: "ml-4",
                                                    div { class: "text-sm font-medium text-gray-900",
                                                        "Alice Johnson"
                                                    }
                                                    div { class: "text-sm text-gray-500",
                                                        "Subscriber"
                                                    }
                                                }
                                            }
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                            "alice@example.com"
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap",
                                            span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800",
                                                "Inactive"
                                            }
                                        }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                            div { class: "flex space-x-2",
                                                button { class: "text-indigo-600 hover:text-indigo-900",
                                                    "Edit"
                                                }
                                                button { class: "text-red-600 hover:text-red-900",
                                                    "Delete"
                                                }
                                                button { class: "text-gray-600 hover:text-gray-900",
                                                    "View"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Bulk Actions
                div { class: "px-4 py-6 bg-white shadow sm:rounded-lg",
                    div { class: "flex flex-col sm:flex-row justify-between items-center space-y-4 sm:space-y-0 sm:space-x-4",
                        div { class: "w-full sm:w-auto",
                            select { class: "block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md",
                                option { "Bulk Actions" }
                                option { "Activate Users" }
                                option { "Deactivate Users" }
                                option { "Delete Users" }
                            }
                        }
                        div { class: "w-full sm:w-auto",
                            button { class: "w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-600 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:w-auto sm:text-sm",
                                "Apply"
                            }
                        }
                        div { class: "w-full sm:w-auto",
                            button { class: "w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:w-auto sm:text-sm",
                                "Export Users"
                            }
                        }
                    }
                }
            }
        }
    }
}