use dioxus::prelude::*;

// Update the path below to the correct module location for components
// wrtiting/ui/components/mod.rs
// wrtiting/desktop/src/views/testing.rs
use ui::{DocumentNode, NodeType, TreeView};


#[component]
pub fn Editor() -> Element {
    let documents = use_signal(|| vec![
        DocumentNode {
            id: "1".to_string(),
            title: "Chapter 1".to_string(),
            children: vec![
                DocumentNode {
                    id: "1-1".to_string(),
                    title: "Opening Scene".to_string(),
                    children: vec![],
                    is_expanded: false,
                    node_type: NodeType::Scene,
                },
                DocumentNode {
                    id: "1-2".to_string(),
                    title: "Research Notes".to_string(),
                    children: vec![],
                    is_expanded: false,
                    node_type: NodeType::Note,
                },
            ],
            is_expanded: true,
            node_type: NodeType::Chapter,
        },
    ]);

    let mut selected_doc = use_signal( || String::new());
    let mut editor_content = use_signal(|| String::new());

    rsx! {
        div { class: "flex h-screen bg-gray-50 text-gray-800",
            // Left Sidebar (Documents)
            Sidebar {
                documents: documents.read().clone(),
                on_select: move |id: String| {
                    selected_doc.set(id.clone());
                    editor_content.set("Sample content for ".to_string() + &id);
                },
            }
            // Right Panel (Editor)
            main { class: "flex-1 flex flex-col overflow-hidden",
                // Editor Toolbar
                header { class: "bg-white border-b border-gray-200 p-2 flex items-center",
                    button { class: "p-2 hover:bg-gray-100 rounded", "Save" }
                    div { class: "ml-auto text-sm text-gray-500",
                        {format!("Words: {}", editor_content.read().split_whitespace().count())}
                    }
                }
                // Main Editor Area
                div { class: "flex-1 overflow-auto bg-white p-6",
                    textarea {
                        class: "w-full h-full p-2 outline-none resize-none",
                        value: editor_content.read().as_str(),
                        oninput: move |e| editor_content.set(e.value().to_string()),
                    }
                }
                // Status Bar
                footer { class: "bg-gray-100 border-t border-gray-200 p-2 text-sm text-gray-500",
                    {format!("Selected: {}", selected_doc.read())}
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
   rsx! {
    aside { class: "w-64 h-full bg-white border-r border-gray-200 flex flex-col",
        // Sidebar Header
        div { class: "p-4 border-b border-gray-200",
            h2 { class: "text-lg font-semibold", "Documents" }
            button { class: "mt-2 w-full bg-blue-500 hover:bg-blue-600 text-white py-1 px-3 rounded text-sm",
                "New Chapter +"
            }
        }
        // Tree View
        div { class: "flex-1 overflow-y-auto p-2",
            TreeView { nodes: documents.clone(), on_select }
        }
        // Sidebar Footer
        div { class: "p-2 border-t border-gray-200 text-sm text-gray-500",
            "Total: {documents.len()} items"
        }
    }
}
}