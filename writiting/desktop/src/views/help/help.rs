// client/components/layout/help_center.rs
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
enum HelpSection {
    Main,
    Faq,
    Contact,
}

#[component]
pub fn HelpCenter() -> Element {
    let mut current_section = use_signal(|| HelpSection::Main);

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-50",
            // Help Center Header
            header { class: "bg-white shadow-sm",
                div { class: "mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8",
                    h1 { class: "text-3xl font-bold text-gray-900", "Help Center" }
                    nav { class: "mt-6 flex space-x-8",
                        HelpNavButton {
                            active: current_section() == HelpSection::Main,
                            onclick: move |_| current_section.set(HelpSection::Main),
                            title: "Getting Started",
                        }
                        HelpNavButton {
                            active: current_section() == HelpSection::Faq,
                            onclick: move |_| current_section.set(HelpSection::Faq),
                            title: "FAQs",
                        }
                        HelpNavButton {
                            active: current_section() == HelpSection::Contact,
                            onclick: move |_| current_section.set(HelpSection::Contact),
                            title: "Contact Us",
                        }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-1",
                div { class: "mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8",
                    match current_section() {
                        HelpSection::Main => rsx! {
                            HelpMain {}
                        },
                        HelpSection::Faq => rsx! {
                            HelpFaq {}
                        },
                        HelpSection::Contact => rsx! {
                            HelpContact {}
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn HelpNavButton(active: bool, onclick: EventHandler, title: String) -> Element {
    rsx! {
        button {
            class: if active { "border-indigo-500 text-indigo-600 whitespace-nowrap border-b-2 px-1 pb-4 text-sm font-medium" } else { "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 px-1 pb-4 text-sm font-medium" },
            onclick: move |_| onclick.call(()),
            "{title}"
        }
    }
}

#[component]
pub fn HelpMain() -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow",
            h2 { class: "text-2xl font-bold mb-4", "Welcome to the Help Center" }
            p { class: "mb-4",
                "Here you'll find resources to help you get the most out of our writing tool."
            }
            div { class: "grid md:grid-cols-2 gap-6 mt-8",
                HelpCard {
                    title: "Getting Started Guide",
                    description: "Learn how to set up and use the basic features of our writing tool.",
                    link: "#",
                }
                HelpCard {
                    title: "Video Tutorials",
                    description: "Watch step-by-step video guides for all major features.",
                    link: "#",
                }
                HelpCard {
                    title: "Keyboard Shortcuts",
                    description: "Boost your productivity with these time-saving shortcuts.",
                    link: "#",
                }
                HelpCard {
                    title: "Feature Overview",
                    description: "Explore all the powerful features available in our writing tool.",
                    link: "#",
                }
            }
        }
    }
}

#[component]
pub fn HelpFaq() -> Element {
    let faqs = vec![
        ("How do I save my work?", "Your work is automatically saved as you type. You can also manually export your documents using the Export menu."),
        ("Can I use Markdown formatting?", "Yes! Our editor supports Markdown formatting. Use the formatting toolbar or type Markdown directly."),
        ("Is there a word count goal feature?", "Absolutely! You can set daily writing goals in the Focus Mode settings."),
        ("How do I enable dark mode?", "Dark mode can be toggled in the application settings or by pressing Ctrl/Cmd + D."),
        ("Is there a mobile app available?", "Currently we have a web application that works on mobile browsers, with a dedicated mobile app coming soon!"),
    ];

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow",
            h2 { class: "text-2xl font-bold mb-6", "Frequently Asked Questions" }
            div { class: "space-y-4",
                for (i , (question , answer)) in faqs.iter().enumerate() {
                    div { key: "{i}", class: "border-b border-gray-200 pb-4",
                        h3 { class: "text-lg font-medium text-gray-900", "{question}" }
                        p { class: "mt-2 text-gray-600", "{answer}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn HelpContact() -> Element {
    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow",
            h2 { class: "text-2xl font-bold mb-6", "Contact Our Support Team" }
            div { class: "grid md:grid-cols-2 gap-8",
                div {
                    h3 { class: "text-lg font-medium mb-4", "Support Options" }
                    ul { class: "space-y-3",
                        li { class: "flex items-start",
                            span { class: "mr-2", "📧" }
                            span {
                                "Email: "
                                a {
                                    class: "text-indigo-600",
                                    href: "mailto:support@writingtool.com",
                                    "support@writingtool.com"
                                }
                            }
                        }
                        li { class: "flex items-start",
                            span { class: "mr-2", "💬" }
                            span { "Live Chat: Available 9am-5pm EST" }
                        }
                        li { class: "flex items-start",
                            span { class: "mr-2", "📞" }
                            span {
                                "Phone: "
                                a {
                                    class: "text-indigo-600",
                                    href: "tel:+18005551234",
                                    "+1 (800) 555-1234"
                                }
                            }
                        }
                    }
                    h3 { class: "text-lg font-medium mt-8 mb-4", "Community Resources" }
                    ul { class: "space-y-3",
                        li { class: "flex items-start",
                            span { class: "mr-2", "💡" }
                            span {
                                "Community Forum: "
                                a { class: "text-indigo-600", href: "#", "forum.writingtool.com" }
                            }
                        }
                        li { class: "flex items-start",
                            span { class: "mr-2", "📚" }
                            span {
                                "Knowledge Base: "
                                a { class: "text-indigo-600", href: "#", "docs.writingtool.com" }
                            }
                        }
                    }
                }
                div {
                    h3 { class: "text-lg font-medium mb-4", "Send Us a Message" }
                    form { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Your Name"
                            }
                            input {
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                                r#type: "text",
                                required: true,
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Email Address"
                            }
                            input {
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                                r#type: "email",
                                required: true,
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Subject"
                            }
                            select { class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                                option { "General Inquiry" }
                                option { "Technical Support" }
                                option { "Feature Request" }
                                option { "Bug Report" }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700",
                                "Message"
                            }
                            textarea {
                                class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500",
                                rows: "4",
                                required: true,
                            }
                        }
                        button {
                            class: "inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                            r#type: "submit",
                            "Send Message"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HelpCard(title: String, description: String, link: &'static str) -> Element {
    rsx! {
        a {
            href: "{link}",
            class: "block p-6 border border-gray-200 rounded-lg hover:border-indigo-300 hover:shadow-md transition-all",
            h3 { class: "text-lg font-medium text-gray-900 mb-2", "{title}" }
            p { class: "text-gray-600", "{description}" }
            div { class: "mt-3 text-indigo-600 font-medium", "Learn more →" }
        }
    }
}