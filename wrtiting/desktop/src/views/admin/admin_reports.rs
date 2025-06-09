
use dioxus::prelude::*;

#[component]
pub fn AdminReports() -> Element {
    rsx! {
        p { "Welcome to the Admin Reports Page!" }
        div { class: "admin-reports",
            h1 { "Admin Reports" }
            p { "Here you can view and manage reported content." }
            div { class: "report-list",
                h2 { "Reported Content" }
                ul {
                    li { "Report 1: Inappropriate content" }
                    li { "Report 2: Spam" }
                    li { "Report 3: Harassment" }
                }
            }
            div { class: "report-actions",
                h2 { "Actions" }
                p { "You can take actions on the reported content." }
                button { class: "btn-approve", "Approve" }
                button { class: "btn-reject", "Reject" }
                button { class: "btn-review", "Review Later" }
            }
            div { class: "report-stats",
                h2 { "Statistics" }
                p { "Total Reports: 50" }
                p { "Resolved Reports: 30" }
                p { "Pending Reviews: 20" }
            }
        }
    }
}