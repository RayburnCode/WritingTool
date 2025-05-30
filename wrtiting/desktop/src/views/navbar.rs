use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn DesktopNavbar(children: Element) -> Element {
    // Get the current route
    let current_route = use_route::<Route>();

    rsx! {
        nav { id: "navbar", class: "w-full text-white shadow-md bg-gray-800",
            div { class: "px-8 py-2 mx-auto flex items-center justify-between",
                // Right side: Links and Button
                div { class: "flex gap-6 items-center text-sm",
                    Link {
                        to: Route::Home {},
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::Home {}) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "Home"
                    }
                    Link {
                        to: Route::Blog { id: 1 },
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::Blog { .. }) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "Blog"
                    }
                    Link {
                        to: Route::TestingView {},
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::TestingView {}) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "Testing"
                    }
                    Link {
                        to: Route::Editor {},
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::Editor {}) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "✍️ Editor"
                    }
                    Link {
                        to: Route::FocusMode {},
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::FocusMode {}) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "🧠 Focus Mode"
                    }
                    Link {
                        to: Route::Settings {},
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::Settings {}) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "⚙️ Settings"
                    }
                    Link {
                        to: Route::HelpMain {},
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::HelpMain {}) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "❓ Help"
                    }
                }
            }
        }
    }
}