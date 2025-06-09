
use dioxus::prelude::*;

#[component]
pub fn ExpenseDashboard() -> Element {
    rsx! {
        p { "Ability to create see the expenses" }
        div { class: "expense-dashboard",
            h1 { "Expense Dashboard" }
            p { "Here you can view and manage your expenses." }
            div { class: "expense-list",
                h2 { "Expense List" }
                ul {
                    li { "Expense 1: $100 - Groceries" }
                    li { "Expense 2: $50 - Transportation" }
                    li { "Expense 3: $200 - Utilities" }
                }
            }
            div { class: "expense-actions",
                h2 { "Actions" }
                p { "You can perform actions on your expenses." }
                button { class: "btn-add", "Add Expense" }
                button { class: "btn-edit", "Edit Expense" }
                button { class: "btn-delete", "Delete Expense" }
            }
            div { class: "expense-stats",
                h2 { "Statistics" }
                p { "Total Expenses: $350" }
                p { "Average Expense: $116.67" }
            }
        }
        div { class: "expense-chart",
            h2 { "Expense Chart" }
            p { "Visual representation of your expenses." }
            // Placeholder for chart component
            div { class: "chart-placeholder", "Chart will be displayed here." }
        }
        div { class: "expense-tips",
            h2 { "Expense Management Tips" }
            ul {
                li { "Track your expenses regularly." }
                li { "Set a budget for each category." }
                li { "Review your spending habits monthly." }
            }
        }
    }
}
