use dioxus::prelude::*;
use crate::Route;
use chrono::Datelike;

#[component]
pub fn Footer() -> Element {
    let year = chrono::Local::now().year();
    
    rsx! {
        footer { class: "bg-gray-50 dark:bg-gray-900 border-t border-gray-200 dark:border-gray-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                // Main footer content - 4 column layout
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-8",
                    // Column 1: Company Info
                    div { class: "space-y-4",
                        div { class: "flex items-center",
                            // Replace with your logo component
                            div { class: "h-8 w-8 bg-blue-600 rounded-full flex items-center justify-center text-white font-bold",
                                "I"
                            }
                            span { class: "ml-2 text-xl font-semibold text-gray-900 dark:text-white",
                                "Intranet"
                            }
                        }
                        p { class: "text-gray-600 dark:text-gray-400 text-sm",
                            "Empowering our team with the tools and resources for success."
                        }
                        div { class: "flex space-x-4",
                            SocialIcon {
                                href: "#",
                                icon: "M22 12c0-5.523-4.477-10-10-10S2 6.477 2 12c0 4.991 3.657 9.128 8.438 9.878v-6.987h-2.54V12h2.54V9.797c0-2.506 1.492-3.89 3.777-3.89 1.094 0 2.238.195 2.238.195v2.46h-1.26c-1.243 0-1.63.771-1.63 1.562V12h2.773l-.443 2.89h-2.33v6.988C18.343 21.128 22 16.991 22 12z",
                            }
                            SocialIcon {
                                href: "#",
                                icon: "M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84",
                            }
                            SocialIcon {
                                href: "#",
                                icon: "M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z",
                            }
                        }
                    }

                    // Column 2: Quick Links
                    div { class: "space-y-2",
                        h3 { class: "text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider",
                            "Quick Links"
                        }
                        FooterLink { href: Route::Home {}, text: "Home" }
                        FooterLinkStr { href: "#", text: "Information Technology" }
                        FooterLinkStr { href: "#", text: "Information Technology" }
                        FooterLinkStr { href: "#", text: "Information Technology" }
                    }

                    // Column 3: Departments
                    div { class: "space-y-2",
                        h3 { class: "text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider",
                            "Departments"
                        }
                        FooterLinkStr { href: "#", text: "Human Resources" }
                        FooterLinkStr { href: "#", text: "Information Technology" }
                        FooterLinkStr { href: "#", text: "Finance" }
                        FooterLinkStr { href: "#", text: "Operations" }
                    }

                    // Column 4: Contact
                    div { class: "space-y-2",
                        h3 { class: "text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider",
                            "Contact"
                        }
                        div { class: "flex items-start",
                            svg {
                                class: "h-5 w-5 text-gray-500 dark:text-gray-400 mr-2 mt-0.5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
                                }
                            }
                            span { class: "text-sm text-gray-600 dark:text-gray-400",
                                "support@company.com"
                            }
                        }
                        div { class: "flex items-start",
                            svg {
                                class: "h-5 w-5 text-gray-500 dark:text-gray-400 mr-2 mt-0.5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z",
                                }
                            }
                            span { class: "text-sm text-gray-600 dark:text-gray-400",
                                "+1 (555) 123-4567"
                            }
                        }
                        div { class: "flex items-start",
                            svg {
                                class: "h-5 w-5 text-gray-500 dark:text-gray-400 mr-2 mt-0.5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z",
                                }
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M15 11a3 3 0 11-6 0 3 3 0 016 0z",
                                }
                            }
                            span { class: "text-sm text-gray-600 dark:text-gray-400",
                                "123 Corporate Dr, City, ST 12345"
                            }
                        }
                    }
                }

                // Copyright notice (full width below columns)
                div { class: "mt-12 pt-8 border-t border-gray-200 dark:border-gray-800",
                    p { class: "text-sm text-gray-500 dark:text-gray-400 text-center",
                        "© {year} Company Name. All rights reserved."
                    }
                }
            }
        }
    }
}

#[component]
fn FooterLink(href: Route, text: &'static str) -> Element {
    rsx! {
        Link {
            to: href,
            class: "text-sm text-gray-600 dark:text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 block",
            {text}
        }
    }
}

#[component]
fn FooterLinkStr(href: &'static str, text: &'static str) -> Element {
    rsx! {
        a {
            href,
            class: "text-sm text-gray-600 dark:text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 block",
            {text}
        }
    }
}

#[component]
fn SocialIcon(href: &'static str, icon: &'static str) -> Element {
    rsx! {
        a {
            href,
            class: "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-white",
            svg {
                class: "h-5 w-5",
                fill: "currentColor",
                view_box: "0 0 24 24",
                path { d: icon }
            }
        }
    }
}