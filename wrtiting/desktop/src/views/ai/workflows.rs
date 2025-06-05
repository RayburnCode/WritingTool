use dioxus::prelude::*;

#[component]
pub fn Workflows() -> Element {
rsx! {
    div {
        h2 { "AI Workflows" }
        p { "Create and manage AI workflows to automate tasks." }
        button { onclick: move |_| println!("Create new workflow"), "Create Workflow" }
        ul {
            li { "Workflow 1" }
            li { "Workflow 2" }
            li { "Workflow 3" }
        }
    }
    div {
        h3 { "Workflow Details" }
        p { "Select a workflow to view or edit its details." }
        select { onchange: move |e| println!("Selected workflow: {}", e.value()),
            option { value: "workflow1", "Workflow 1" }
            option { value: "workflow2", "Workflow 2" }
            option { value: "workflow3", "Workflow 3" }
        }
        button { onclick: move |_| println!("Edit workflow"), "Edit Workflow" }
    }
}
}




