
use dioxus::prelude::*;

#[component]
pub fn AdminUsers() -> Element {
    rsx! {
        p { "Welcome to the Admin Users Page!" }
        div { class: "admin-users",
            h1 { "Admin Users" }
            p { "Here you can manage users, view their profiles, and perform administrative tasks." }
            div { class: "user-list",
                h2 { "User List" }
                ul {
                    li { "User 1: John Doe" }
                    li { "User 2: Jane Smith" }
                    li { "User 3: Alice Johnson" }
                }
            }
            div { class: "user-actions",
                h2 { "Actions" }
                p { "You can perform actions on the users." }
                button { class: "btn-edit", "Edit User" }
                button { class: "btn-delete", "Delete User" }
                button { class: "btn-view-profile", "View Profile" }
            }
            div { class: "user-stats",
                h2 { "User Statistics" }
                p { "Total Users: 1000" }
                p { "Active Users: 800" }
                p { "Inactive Users: 200" }
            }
        }
    }
}