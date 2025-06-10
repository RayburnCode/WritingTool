use dioxus::prelude::*;
use crate::Avatar;

#[derive(Props, Clone, PartialEq)]
pub struct NavbarDropProps {

    nav_menu_items: Vec<NavMenuItem>,
}

#[derive(Clone, PartialEq)]
pub struct NavMenuItem {
    pub label: String,
    pub to: Option<String>,  
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn NavDrop(props: NavbarDropProps) -> Element {
    let mut show_dropdown = use_signal(|| false);

    
    // Close when clicking anywhere (including outside)
    use_dropdown_auto_close(show_dropdown);

    rsx! {
        div { class: "relative",

            // Clickable avatar
            div {
                onclick: move |_| show_dropdown.set(!show_dropdown()),
                class: "cursor-pointer",
                p { "Additional" }
            }

            // Dropdown menu
            if show_dropdown() {
                div { class: "absolute right-0 z-50 mt-2 w-48 rounded-md shadow-lg bg-white dark:bg-gray-700",
                    // User info

                    // Menu items
                    ul { class: "py-1",
                        for item in props.nav_menu_items.iter().cloned() {
                            li {
                                if let Some(to) = item.to.clone() {
                                    Link {
                                        to,
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                        onclick: move |evt| {
                                            show_dropdown.set(false);
                                            if let Some(handler) = item.onclick.clone() {
                                                handler.call(evt);
                                            }
                                        },
                                        {item.label.as_str()}
                                    }
                                } else {
                                    span {
                                        class: "block px-4 py-2 text-sm text-gray-700 dark:text-gray-200",
                                        onclick: move |evt| {
                                            show_dropdown.set(false);
                                            if let Some(handler) = item.onclick.clone() {
                                                handler.call(evt);
                                            }
                                        },
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

// Simple auto-close hook without web-sys
fn use_dropdown_auto_close(mut show_dropdown: Signal<bool>) {
    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            if show_dropdown() {
                if let Some(clicked) = try_get_click_event().await {
                    if !clicked.within_dropdown {
                        show_dropdown.set(false);
                    }
                }
            }
        }
    });
}

// Mock implementation - in a real app you'd use proper event handling
async fn try_get_click_event() -> Option<ClickEvent> {
    None
}

struct ClickEvent {
    within_dropdown: bool,
}