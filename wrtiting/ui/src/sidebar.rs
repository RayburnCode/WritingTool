use dioxus::prelude::*;

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