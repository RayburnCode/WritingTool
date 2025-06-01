/// A shared state for the theme, which can be used across components
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

pub fn use_theme(cx: &ScopeState) -> &UseSharedState<Theme> {
    use_context::<UseSharedState<Theme>>().expect("Theme context not provided")
}


    use_shared_state_provider(|| Theme::Dark); // Default to dark
