// pg_app/components/src/ui/avatar_drop.rs
use dioxus::prelude::*;
use crate::Avatar;
// use super::routes::Route;  


#[component]
pub fn AvatarDrop() -> Element {
    let mut show_dropdown: Signal<bool> = use_signal(|| false);

    let toggle_dropdown = move |_| {
        show_dropdown.set(!show_dropdown());
    };

    rsx!(
        div { class: "flex justify-end items-center ",
            div { class: "relative",
                // Make the Avatar clickable
                div { onclick: toggle_dropdown, class: "cursor-pointer", Avatar {
                } }

                // Dropdown menu that appears when show_dropdown is true
                if show_dropdown() {
                    div { class: "absolute right-0 z-50 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-700",
                        div { class: "px-4 py-3 border-b border-gray-200 dark:border-gray-600",
                            span { class: "block text-sm text-gray-900 dark:text-white",
                                "Bonnie Green"
                            }
                            span { class: "block text-sm text-gray-500 truncate dark:text-gray-400",
                                "name@flowbite.com"
                            }
                        }
                        ul { class: "py-1",
                            li {
                                a {
                                    href: "#",
                                    class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                    "Dashboard"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                    "Settings"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                    "Sign out"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}