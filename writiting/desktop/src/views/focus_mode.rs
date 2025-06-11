// client/components/layout/focus_mode.rs
use dioxus::prelude::*;

#[component]
pub fn FocusMode() -> Element {
    let mut text = use_signal(|| String::new());
    let mut word_count = use_signal(|| 0);
    let mut char_count = use_signal(|| 0);
    let mut is_fullscreen = use_signal(|| false);
    let mut font_size = use_signal(|| 16);
    let mut theme = use_signal(|| "light".to_string());

    // Calculate word and character count whenever text changes
    use_effect(move || {
        let content = text();
        word_count.set(content.split_whitespace().count());
        char_count.set(content.chars().count());
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-{theme} transition-colors duration-200",
            if !is_fullscreen() {
                header { class: "flex justify-between items-center p-4 border-b",
                    h1 { class: "text-xl font-semibold", "Focus Mode" }
                    div { class: "flex gap-4",
                        button {
                            class: "p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700",
                            onclick: move |_| is_fullscreen.toggle(),
                                                // Icon { icon: Shape::ArrowsPointingOut }
                        }
                        select {
                            class: "p-2 rounded bg-transparent border",
                            onchange: move |e| theme.set(e.value()),
                            option { value: "light", "Light" }
                            option { value: "dark", "Dark" }
                        }
                        select {
                            class: "p-2 rounded bg-transparent border",
                            onchange: move |e| font_size.set(e.value().parse().unwrap_or(16)),
                            option { value: "14", "Small" }
                            option { value: "16", "Medium" }
                            option { value: "18", "Large" }
                            option { value: "20", "Extra Large" }
                        }
                    }
                }
            }

            main { class: "flex-1 flex flex-col",
                textarea {
                    class: "flex-1 p-6 resize-none outline-none bg-transparent",
                    style: "font-size: {font_size}px;",
                    placeholder: "Start writing here...",
                    value: "{text}",
                    oninput: move |e| text.set(e.value()),
                    autofocus: true,
                    spellcheck: true,
                }

                if !is_fullscreen() {
                    footer { class: "p-2 border-t flex justify-between text-sm text-gray-500",
                        div { "Words: {word_count}" }
                        div { "Characters: {char_count}" }
                    }
                }
            }
        }
    }
}