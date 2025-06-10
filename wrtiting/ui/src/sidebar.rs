use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SidebarItem {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub children: Vec<SidebarItem>,
    pub is_expanded: bool,
}

#[component]
pub fn TraditionalSidebar(
    items: Vec<SidebarItem>,
    selected_item: String,
    on_select: EventHandler<String>,
) -> Element {
    let mut is_collapsed = use_signal(|| false);
    let mut sidebar_width = use_signal(|| 16); // Default width in rem
    
    rsx! {
        div {
            class: "flex flex-col h-full bg-gray-50 border-r border-gray-200 transition-all duration-200 ease-in-out",
            style: format!(
                "width: {}rem; min-width: {}rem;",
                if *is_collapsed.read() { 4 } else { *sidebar_width.read() },
                if *is_collapsed.read() { 4 } else { 16 },
            ),
            // Sidebar header
            div { class: "flex items-center justify-between p-4 border-b border-gray-200",
                // Logo/Title (hidden when collapsed)
                if !*is_collapsed.read() {
                    h1 { class: "text-xl font-semibold text-gray-800", "My App" }
                } else {
                    div { class: "w-6 h-6" } // Spacer
                }
                // Collapse button
                button {
                    class: "p-1 rounded-md hover:bg-gray-200",
                    onclick: move |_| is_collapsed.toggle(),
                    if *is_collapsed.read() {
                        "→"
                        if *is_collapsed.read() {
                            "→"
                        } else {
                            "←"
                        }
                    }
                }
                // Navigation items
                nav { class: "flex-1 overflow-y-auto py-2",
                    ul { class: "space-y-1",
                        {items.iter().map(|item| rsx! {
                            SidebarNavItem {
                                key: "{item.id}",
                                item: item.clone(),
                                selected_item: selected_item.clone(),
                                on_select,
                                level: 0,
                                is_collapsed: *is_collapsed.read(),
                            }
                        })}
                    }
                }
                // User section (bottom of sidebar)
                div { class: "p-4 border-t border-gray-200",
                    if !*is_collapsed.read() {
                        div { class: "flex items-center gap-3",
                            div { class: "w-8 h-8 rounded-full bg-gray-300 flex items-center justify-center",
                                "👤"
                            }
                            div { class: "flex-1",
                                p { class: "font-medium", "John Doe" }
                                p { class: "text-xs text-gray-500", "Admin" }
                            }
                        }
                    } else {
                        div { class: "w-8 h-8 rounded-full bg-gray-300 flex items-center justify-center mx-auto",
                            "👤"
                        }
                    }
                }
            }
        }
    }}


#[component]
pub fn SidebarNavItem(
    item: SidebarItem,
    selected_item: String,
    on_select: EventHandler<String>,
    level: usize,
    is_collapsed: bool,
) -> Element {
    let mut is_expanded = use_signal(|| item.is_expanded);
    let has_children = !item.children.is_empty();
    
    let is_active = selected_item == item.id;
    
    let padding_left = if is_collapsed {
        "pl-2".to_string()
    } else {
        format!("pl-{}", level * 4 + 2)
    };
    
    rsx! {
        li { class: "select-none",
            div {
                class: format!(
                    "flex items-center {} pr-2 py-2 rounded-r-lg mx-2 hover:bg-gray-200 transition-colors duration-150 {}",
                    padding_left,
                    if is_active { "bg-blue-100 text-blue-600" } else { "text-gray-700" },
                ),
                onclick: move |_| {
                    if has_children {
                        is_expanded.toggle();
                    }
                    on_select.call(item.id.clone());
                },

                // Icon
                span { class: "flex-shrink-0 mr-3 text-lg",
                    if is_collapsed && has_children && !*is_expanded.read() {
                        "⊕"
                    } else {
                        "{item.icon}"
                    }
                }

                // Title (hidden when collapsed)
                if !is_collapsed {
                    span { class: "flex-1 truncate", "{item.title}" }

                    // Expand/collapse icon (if has children)
                    if has_children {
                        span { class: "ml-2 text-gray-500",
                            if *is_expanded.read() {
                                "▾"
                            } else {
                                "▸"
                            }
                        }
                    }
                    {item.children.iter().map(|child| rsx! {
                        SidebarNavItem {
                            key: "{child.id}",
                            item: child.clone(),
                            selected_item: selected_item.clone(),
                            on_select,
                            level: level + 1,
                            is_collapsed,
                        }
                    })}
                }
            }
        }
    }
}
