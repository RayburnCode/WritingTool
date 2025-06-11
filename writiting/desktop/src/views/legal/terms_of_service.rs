use dioxus::prelude::*;

#[component]
pub fn TermsOfService() -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 font-sans",
            // Toggle button
            button {
                class: "bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors",
                onclick: move |_| {
                    let open = *is_open.read();
                    is_open.set(!open)
                },
                if *is_open.read() {
                    "Hide Terms of Service"
                } else {
                    "Show Terms of Service"
                }
            }
            // Terms content (conditionally rendered)
            if *is_open.read() {
                div { class: "mt-6 p-6 bg-gray-50 rounded-lg shadow-md",
                    h2 { class: "text-2xl font-bold text-gray-800 pb-3 border-b border-gray-200",
                        "Terms of Service"
                    }
                    p { class: "text-gray-600 mt-2",
                        {format_args!("Last Updated: {}", chrono::Local::now().format("%Y-%m-%d"))}
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "1. Acceptance of Terms" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "By accessing or using our service, you agree to be bound by these Terms. 
                        If you disagree with any part of the terms, you may not access the service."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "2. User Responsibilities" }
                    p { class: "text-gray-600 mt-2", "As a user of our service, you agree to:" }
                    ul { class: "list-disc pl-6 text-gray-600 space-y-1 mt-2",
                        li { "Use the service only for lawful purposes" }
                        li { "Not violate any applicable laws or regulations" }
                        li { "Not transmit any harmful or malicious content" }
                        li { "Maintain the confidentiality of your account credentials" }
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "3. Intellectual Property" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "The service and its original content, features, and functionality are and will 
                        remain the exclusive property of our company and its licensors."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "4. Termination" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "We may terminate or suspend access to our service immediately, without prior 
                        notice or liability, for any reason whatsoever, including without limitation 
                        if you breach the Terms."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6",
                        "5. Limitation of Liability"
                    }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "In no event shall we be liable for any indirect, incidental, special, 
                        consequential or punitive damages arising out of your use of the service."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "6. Governing Law" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "These Terms shall be governed by and construed in accordance with the laws of 
                        [Your Country/State], without regard to its conflict of law provisions."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "7. Changes to Terms" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "We reserve the right to modify these terms at any time. Your continued use 
                        of the service after any such changes constitutes your acceptance of the new Terms."
                    }
                    div { class: "flex justify-end mt-6",
                        button {
                            class: "bg-red-500 hover:bg-red-600 text-white font-medium py-2 px-4 rounded-lg transition-colors",
                            onclick: move |_| is_open.set(false),
                            "Close"
                        }
                    }
                }
            }
        }
    }
}