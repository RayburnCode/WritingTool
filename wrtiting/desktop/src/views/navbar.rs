// writing/desktop/src/views/navbar.rs
use dioxus::prelude::*;
use crate::{
    state::Theme, 
    Route,
};

use ui::{
    AvatarDrop, 
    MenuItem, NavDrop, NavMenuItem, Search
};

#[component]
pub fn DesktopNavbar(children: Element) -> Element {
    // Get the current route
    let current_route = use_route::<Route>();

    let mut theme = use_signal(|| Theme::Light);
    let dark_mode = *theme.read() == Theme::Dark;

    let toggle_theme = move |_: Event<MouseData>| {
        let new_theme = if dark_mode { Theme::Light } else { Theme::Dark };
        *theme.write() = new_theme;
    };

  let menu_items = vec![

        MenuItem {
            label: "Profile".to_string(),
            to: Some("/profile".to_string()),  // Use string paths
            onclick: None,
        },
        MenuItem {
            label: "Account Settings".to_string(),
            to: Some("/settings".to_string()),
            onclick: None,
        },
                MenuItem {
            label: "Toggle Theme - DNW".to_string(),
            to: None,
            onclick: Some(EventHandler::new(toggle_theme)),
        },
            MenuItem {
            label: "Help Center".to_string(),
            to: Some("/help".to_string()),
            onclick: None,
        },
            MenuItem {
            label: "Sign out".to_string(),
            to: Some("#".to_string()),
            onclick: Some(EventHandler::new(|_| {
                // Handle sign out
            })),
        },
    ];


    

  let nav_menu_items = vec![

        NavMenuItem {
            label: "Login".to_string(),
            to: Some("/auth/login".to_string()),  // Use string paths
            onclick: None,
        },
        NavMenuItem {
            label: "Register".to_string(),
            to: Some("/auth/register".to_string()),
            onclick: None,
        },
                NavMenuItem {
            label: "Reset Password".to_string(),
            to: Some("/auth/reset-password".to_string()),
            onclick: None,
        },
            NavMenuItem {
            label: "Admin Dashboard".to_string(),
            to: Some("/admin".to_string()),
            onclick: None,
        },
             NavMenuItem {
            label: "Admin Users".to_string(),
            to: Some("/admin/users".to_string()),
            onclick: None,
        },
            NavMenuItem {
            label: "Admin Reports".to_string(),
            to: Some("/admin/reports".to_string()),
            onclick: None,
        },
                NavMenuItem {
            label: "Admin Settings".to_string(),
            to: Some("/admin/settings".to_string()),
            onclick: None,
        },

            NavMenuItem {
            label: "AI Chat".to_string(),
            to: Some("/ai".to_string()),
            onclick: None,
        },

                    NavMenuItem {
            label: "Terms of Service".to_string(),
            to: Some("/legal/terms".to_string()),
            onclick: None,
        },

                    NavMenuItem {
            label: "Privacy Policy".to_string(),
            to: Some("/legal/privacy".to_string()),
            onclick: None,
        },
   
    ];



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
                        to: Route::Editor {},
                        class: format!(
                            "hover:text-blue-400 transition {}",
                            if matches!(current_route, Route::Editor {}) {
                                "text-blue-400 font-medium border-b-2 border-blue-400"
                            } else {
                                "text-gray-300"
                            },
                        ),
                        "Editor"
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
                        "Focus Mode"
                    }
                    NavDrop { nav_menu_items }
                    Search {}
                    AvatarDrop {
                        name: "Bonnie Testing",
                        email: "name@test.com",
                        menu_items,
                    }
                }
            }
        }
    }
}