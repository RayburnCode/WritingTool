
use dioxus::prelude::*;

#[component]
pub fn AdminDashboard() -> Element {
    rsx! {
        p { "Welcome to the Admin Dashboard!" }
        div { class: "admin-dashboard",
            h1 { "Admin Dashboard" }
            p { "Here you can manage users, view reports, and perform administrative tasks." }
        }
        div { class: "admin-actions",
            ul {
                li {
                    a { href: "/admin/users", "Manage Users" }
                }
                li {
                    a { href: "/admin/reportes", "View Reports" }
                }
                li {
                    a { href: "/admin/settings", "Settings" }
                }
            }
        }
        div { class: "admin-stats",
            h2 { "Statistics" }
            p { "Total Users: 1000" }
            p { "Reported Content: 50" }
            p { "Active Sessions: 200" }
        }
    }
}
