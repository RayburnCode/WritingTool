use dioxus::prelude::*;
use dioxus_desktop::use_window;


#[derive(Clone, Debug, PartialEq)]
pub struct DocumentNode {
    pub id: String,
    pub title: String,
    pub children: Vec<DocumentNode>,
    pub is_expanded: bool,
    pub node_type: NodeType, // Add this for icons
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Chapter,
    Scene,
    Note,
}

#[component]
pub fn TreeView( nodes: Vec<DocumentNode>, on_select: EventHandler<String>) -> Element {
    rsx! {
        ul { class: "space-y-1",
            {nodes.iter().map(|node| rsx! {
                TreeNode {
                    key: "{node.id}",
                    node: node.clone(),
                    on_select,
                    level: 0,
                }
            })}
        }
    }
}

#[component]
pub fn TreeNode(
    node: DocumentNode,
    on_select: EventHandler<String>,
    level: usize,
) -> Element {
    let mut is_expanded = use_signal(|| node.is_expanded);
    
    let icon = match node.node_type {
        NodeType::Chapter => "📖",
        NodeType::Scene => "🎬",
        NodeType::Note => "📝",
    };

    let padding_left = format!("pl-{}", level * 4 + 2);

    rsx! {
        li { class: "select-none",
            div {
                class: "flex items-center hover:bg-gray-100 rounded-md py-1 pr-2 {padding_left}",
                onclick: move |_| {
                    let expanded = *is_expanded.read();
                    is_expanded.set(!expanded);
                    on_select.call(node.id.clone());
                },
                // Expand/collapse icon
                if !node.children.is_empty() {
                    span { class: "mr-1 text-gray-500",
                        {if *is_expanded.read() { "▾" } else { "▸" }}
                    }
                } else {
                    span { class: "w-4" }
                }
                // Node icon and title
                span { class: "mr-2", "{icon}" }
                span { class: "flex-1 truncate", "{node.title}" }
                // Add button
                button {
                    class: "ml-auto p-1 text-gray-400 hover:text-gray-600 hover:bg-gray-200 rounded",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    "+"
                }
            }
            // Children - removed the duplicate if statement
            if *is_expanded.read() && !node.children.is_empty() {
                ul { class: "space-y-1",
                    {node.children.iter().map(|child| rsx! {
                        TreeNode {
                            key: "{child.id}",
                            node: child.clone(),
                            on_select,
                            level: level + 1,
                        }
                    })}
                }
            }
        }
    }
}




#[component]
pub fn Sidebar(
    documents: Vec<DocumentNode>,
    on_select: EventHandler<String>,
) -> Element {
    let mut is_open = use_signal(|| true);
    let mut sidebar_width: Signal<i32> = use_signal(|| 14); // Default width in rem
    let mut is_resizing = use_signal(|| false);
    let mut start_x = use_signal(|| 0.0);
    let mut start_width = use_signal(|| 14.0);
    let window = use_window();
    let window1 = window.clone();
    let window2 = window.clone();
    let window3 = window.clone();

    rsx! {
        div {
            class: "flex h-full",
            style: format!("width: {}rem", if is_open() { sidebar_width() } else { 0 }),
            // Sidebar Content
            onmouseup: move |_| {
                if is_resizing() {
                    is_resizing.set(false);
                    let _ = window1.set_cursor_grab(false);
                    let _ = window1.set_cursor_visible(true);
                }
            },
            onmouseleave: move |_| {
                if is_resizing() {
                    is_resizing.set(false);
                    let _ = window2.set_cursor_grab(false);
                    let _ = window2.set_cursor_visible(true);
                }
            },
            onmousemove: move |e| {
                if is_resizing() {
                    let new_width = (start_width()
                        + (e.data.element_coordinates().x - start_x()))
                        .max(150.0)
                        .min(500.0);
                    sidebar_width.set(new_width as i32);
                }
            },
            // Collapse/Expand Button
            button {
                class: "absolute z-10 top-4 left-0 w-4 h-8 bg-gray-300 hover:bg-gray-400 rounded-r-md shadow transition-all duration-300 ease-in-out",
                style: format!("left: {}rem; top: 5rem;", if is_open() { sidebar_width() } else { 0 }),
                onclick: move |_| is_open.toggle(),
                {if is_open() { "◄" } else { "►" }}
            }

            // Sidebar Header
            div { class: "p-4 border-b border-gray-200",
                h2 { class: "text-lg font-semibold", "Documents" }
                div { class: "flex gap-2",
                    button {
                        class: "text-xs bg-gray-200 hover:bg-gray-300 py-1 px-2 rounded",
                        onclick: move |_| is_open.toggle(),
                        {if is_open() { "Hide" } else { "Show" }}
                    }
                }
                // Tree View
                div { class: "flex-1 overflow-y-auto p-2",
                    TreeView { nodes: documents.clone(), on_select }
                }
            }

            // Sidebar Resizer
            div {
                class: "absolute top-0 right-0 h-full w-2 cursor-col-resize z-20",
                onmousedown: move |e: dioxus::events::MouseEvent| {
                    is_resizing.set(true);
                    start_x.set(e.data.element_coordinates().x);
                    start_width.set(sidebar_width() as f64);
                    let _ = window3.set_cursor_grab(true);
                    let _ = window3.set_cursor_visible(false);
                },
            }
        }
    }
    }
