use dioxus::prelude::*;

#[component]
pub fn Modal(
    is_open: bool,
    on_close: EventHandler,
    title: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: if is_open { "fixed inset-0 z-50 overflow-y-auto" } else { "hidden" },
            // Semi-transparent backdrop that darkens the background
            div {
                class: "fixed inset-0 bg-gray-100 bg-opacity-30 backdrop-blur-sm transition-opacity",
                "aria-hidden": "true",
                onclick: move |_| on_close.call(()),
            }
            // Modal container
            div { class: "flex min-h-full items-center justify-center p-4 text-center",
                div { class: "relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg",
                    // Modal content (fully opaque)
                    div { class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                        h3 { class: "text-lg font-medium leading-6 text-gray-900 mb-4",
                            "{title}"
                        }
                        {children}
                    }
                    // Modal footer
                    div { class: "bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6",
                        button {
                            r#type: "button",
                            class: "mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm",
                            onclick: move |_| on_close.call(()),
                            "Close"
                        }
                    }
                }
            }
        }
    }
}