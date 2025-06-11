use dioxus::prelude::*;

#[component]
pub fn PrivacyPolicy() -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 font-sans",
            // Toggle button
            button {
                class: "bg-green-600 hover:bg-green-700 text-white font-medium py-2 px-4 rounded-lg transition-colors",
                onclick: move |_| is_open.set(!is_open()),
                if is_open() {
                    "Hide Privacy Policy"
                } else {
                    "Show Privacy Policy"
                }
            }
            // Policy content (conditionally rendered)
            if is_open() {
                div { class: "mt-6 p-6 bg-gray-50 rounded-lg shadow-md",
                    h2 { class: "text-2xl font-bold text-gray-800 pb-3 border-b border-gray-200",
                        "Privacy Policy"
                    }
                    p { class: "text-gray-600 mt-2",
                        {format!("Last Updated: {}", chrono::Local::now().format("%Y-%m-%d"))}
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6",
                        "1. Information We Collect"
                    }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "We collect minimal personal information necessary to provide our services. 
                        This may include your name, email address, and usage data when you interact with our application."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6",
                        "2. How We Use Your Information"
                    }
                    p { class: "text-gray-600 mt-2", "The information we collect is used to:" }
                    ul { class: "list-disc pl-6 text-gray-600 space-y-1 mt-2",
                        li { "Provide and maintain our service" }
                        li { "Notify you about changes to our service" }
                        li { "Improve user experience" }
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "3. Data Security" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "We implement appropriate technical and organizational measures to protect 
                        your personal data against unauthorized access, alteration, or destruction."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "4. Third-Party Services" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "We may employ third-party companies to facilitate our service. 
                        These third parties have access only to the information needed to perform their functions."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6",
                        "5. Changes to This Policy"
                    }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "We may update our Privacy Policy from time to time. 
                        We will notify you of any changes by posting the new policy on this page."
                    }
                    h3 { class: "text-xl font-semibold text-gray-700 mt-6", "6. Contact Us" }
                    p { class: "text-gray-600 mt-2 leading-relaxed",
                        "If you have any questions about this Privacy Policy, please contact us."
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