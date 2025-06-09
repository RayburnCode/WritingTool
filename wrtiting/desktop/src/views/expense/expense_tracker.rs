
use dioxus::prelude::*;

#[component]
pub fn ExpenseTrackerList() -> Element {
    rsx! {
        p { "Ability to track the expenses in a Table Format" }

        div { class: "expense-tracker-list",
            h1 { "Expense Tracker List" }
            p { "Here you can view and manage your expenses in a table format." }
            table {
                thead {
                    tr {
                        th { "Date" }
                        th { "Category" }
                        th { "Amount" }
                        th { "Description" }
                    }
                }
                tbody {
                    tr {
                        td { "2023-10-01" }
                        td { "Groceries" }
                        td { "$100" }
                        td { "Weekly grocery shopping" }
                    }
                    tr {
                        td { "2023-10-02" }
                        td { "Transportation" }
                        td { "$50" }
                        td { "Bus fare" }
                    }
                    tr {
                        td { "2023-10-03" }
                        td { "Utilities" }
                        td { "$200" }
                        td { "Electricity bill" }
                    }
                }
            }
        }
        div { class: "expense-tracker-actions",
            h2 { "Actions" }
            p { "You can perform actions on your expenses." }
            button { class: "btn-add", "Add Expense" }
            button { class: "btn-edit", "Edit Expense" }
            button { class: "btn-delete", "Delete Expense" }
            button { class: "btn-export", "Export to CSV" }
            button { class: "btn-import", "Import from CSV" }
        }
    }
}
