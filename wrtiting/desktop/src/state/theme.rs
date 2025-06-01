// writing/desktop/src/state/theme.rs
use dioxus::prelude::*;

/// Theme variants (Dark/Light)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

/// Initialize the theme provider (call this in `main.rs`)
pub fn init_theme() -> Signal<Theme> {
    // Provide a Signal<Theme> to the component tree (default: Dark)
    use_hook(|| Signal::new(Theme::Dark))
}

/// Hook to access the theme Signal from any component
pub fn use_theme() -> Signal<Theme> {
    consume_context::<Signal<Theme>>()
}