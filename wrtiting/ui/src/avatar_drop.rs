// pg_app/components/src/ui/avatar_drop.rs
use dioxus::prelude::*;
use crate::Avatar;


#[derive(Props, Clone, PartialEq)]
pub struct AvatarDropProps {
    // User information
    name: String,
    email: String,
    
    // Dropdown menu items
    menu_items: Vec<MenuItem>,
}

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub route: Option<Route>,  // Changed from href to route
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn AvatarDrop(props: AvatarDropProps) -> Element {
    let mut show_dropdown: Signal<bool> = use_signal(|| false);

    let toggle_dropdown = move |_| {
        show_dropdown.set(!show_dropdown());
    };

    rsx! {
        div { class: "flex justify-end items-center ",
            div { class: "relative",
                // Make the Avatar clickable
                div { onclick: toggle_dropdown, class: "cursor-pointer", Avatar {} }

                // Dropdown menu
                if show_dropdown() {
                    div { class: "absolute right-0 z-50 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-700",
                        div { class: "px-4 py-3 border-b border-gray-200 dark:border-gray-600",
                            span { class: "block text-sm text-gray-900 dark:text-white",
                                {props.name.clone()}
                            }
                            span { class: "block text-sm text-gray-500 truncate dark:text-gray-400",
                                {props.email.clone()}
                            }
                        }
                        ul { class: "py-1",
                            for item in props.menu_items.iter() {
                                li {
                                    // Use Link for route-based navigation
                                    if let Some(route) = &item.route {
                                        Link {
                                            to: route.clone(),
                                            class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                            onclick: move |evt| {
                                                if let Some(handler) = &item.onclick {
                                                    handler.call(evt);
                                                }
                                            },
                                            {item.label.as_str()}
                                        }
                                    } else {
                                        // Fallback to regular anchor for non-route items
                                        a {
                                            href: "#",
                                            onclick: move |evt| {
                                                if let Some(handler) = &item.onclick {
                                                    handler.call(evt);
                                                }
                                                evt.prevent_default();
                                            },
                                            class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                            {item.label.as_str()}
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