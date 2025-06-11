use dioxus::prelude::*;

#[component]
pub fn AdminReports() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100",
            // Header
            header { class: "bg-white shadow",
                div { class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Admin Reports" }
                    p { class: "mt-1 text-gray-600", "Here you can view and manage reported content." }
                }
            }

            // Main Content
            main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
                // Welcome Banner
                div { class: "px-4 pb-6 sm:px-0",
                    div { class: "bg-blue-50 border-l-4 border-blue-400 p-4",
                        div { class: "flex items-start",
                            div { class: "flex-shrink-0",
                                svg {
                                    class: "h-5 w-5 text-blue-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 20 20",
                                    fill: "currentColor",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2h-1V9z",
                                        clip_rule: "evenodd",
                                    }
                                }
                            }
                            div { class: "ml-3",
                                p { class: "text-sm text-blue-700",
                                    "Welcome to the Admin Reports Page! Review and take action on reported content below."
                                }
                            }
                        }
                    }
                }

                // Stats Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 px-4 py-6",
                    // Total Reports Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Total Reports" }
                            p { class: "mt-1 text-3xl font-semibold text-indigo-600",
                                "50"
                            }
                        }
                    }
                    // Resolved Reports Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Resolved Reports" }
                            p { class: "mt-1 text-3xl font-semibold text-green-600",
                                "30"
                            }
                        }
                    }
                    // Pending Reviews Card
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",
                            h3 { class: "text-lg font-medium text-gray-900", "Pending Reviews" }
                            p { class: "mt-1 text-3xl font-semibold text-yellow-600",
                                "20"
                            }
                        }
                    }
                }

                // Reports List
                div { class: "px-4 py-6",
                    div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                        div { class: "px-4 py-5 sm:px-6 border-b border-gray-200",
                            h2 { class: "text-lg font-medium leading-6 text-gray-900",
                                "Reported Content"
                            }
                        }
                        ul { class: "divide-y divide-gray-200",
                            li { class: "px-4 py-4 sm:px-6",
                                div { class: "flex items-center justify-between",
                                    p { class: "text-sm font-medium text-indigo-600 truncate",
                                        "Report 1: Inappropriate content"
                                    }
                                    div { class: "ml-2 flex-shrink-0 flex",
                                        span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800",
                                            "Urgent"
                                        }
                                    }
                                }
                            }
                            li { class: "px-4 py-4 sm:px-6",
                                div { class: "flex items-center justify-between",
                                    p { class: "text-sm font-medium text-indigo-600 truncate",
                                        "Report 2: Spam"
                                    }
                                    div { class: "ml-2 flex-shrink-0 flex",
                                        span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-yellow-100 text-yellow-800",
                                            "Medium"
                                        }
                                    }
                                }
                            }
                            li { class: "px-4 py-4 sm:px-6",
                                div { class: "flex items-center justify-between",
                                    p { class: "text-sm font-medium text-indigo-600 truncate",
                                        "Report 3: Harassment"
                                    }
                                    div { class: "ml-2 flex-shrink-0 flex",
                                        span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800",
                                            "Urgent"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Action Buttons
                div { class: "px-4 py-6 grid grid-cols-1 gap-4 sm:grid-cols-3",
                    button { class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-md text-center transition duration-150 ease-in-out",
                        "Approve"
                    }
                    button { class: "bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-md text-center transition duration-150 ease-in-out",
                        "Reject"
                    }
                    button { class: "bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-2 px-4 rounded-md text-center transition duration-150 ease-in-out",
                        "Review Later"
                    }
                }
            }
        }
    }
}