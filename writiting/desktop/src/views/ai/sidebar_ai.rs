use dioxus::prelude::*;
use ui::{TraditionalSidebar, SidebarItem};

#[component]
pub fn AISidebar() -> Element {
    let mut selected_item = use_signal(|| "dashboard".to_string());
    
    let sidebar_items = vec![
        SidebarItem {
            id: "dashboard".to_string(),
            title: "Dashboard".to_string(),
            icon: "📊".to_string(),
            children: vec![],
            is_expanded: false,
        },
        SidebarItem {
            id: "projects".to_string(),
            title: "Projects".to_string(),
            icon: "📁".to_string(),
            is_expanded: true,
            children: vec![
                SidebarItem {
                    id: "project-1".to_string(),
                    title: "Project 1".to_string(),
                    icon: "📄".to_string(),
                    children: vec![],
                    is_expanded: false,
                },
                SidebarItem {
                    id: "project-2".to_string(),
                    title: "Project 2".to_string(),
                    icon: "📄".to_string(),
                    children: vec![],
                    is_expanded: false,
                },
            ],
        },
        // Add more items as needed
    ];
    
    rsx! {
        div { class: "flex h-screen",
            TraditionalSidebar {
                items: sidebar_items,
                selected_item: selected_item(),
                on_select: move |id| selected_item.set(id),
            }
            div { class: "flex-1 bg-white",
                // Main content goes here
                "Selected: {selected_item}"
            }
        }
    }
}