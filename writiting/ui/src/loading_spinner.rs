#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpinnerSize {
    #[default]
    Medium,
    Small,
    Large,
}

impl SpinnerSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Small => "w-4 h-4",
            Self::Medium => "w-8 h-8",
            Self::Large => "w-12 h-12",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpinnerColor {
    #[default]
    Gray,
    Light,
    Dark,
    Blue,
    Red,
}

impl SpinnerColor {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Gray => "text-gray-600",
            Self::Light => "text-gray-200",
            Self::Dark => "text-gray-800",
            Self::Blue => "text-blue-500",
            Self::Red => "text-red-500",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SpinnerProps {
    #[props(default)]
    size: SpinnerSize,
    #[props(default)]
    color: SpinnerColor,
    class: Option<String>,
    label: Option<String>,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let SpinnerProps { size, color, class, label } = props;
    
    let spinner_class = format!(
        "inline-block animate-spin border-4 border-solid border-current border-r-transparent rounded-full {} {} {}",
        size.to_class(),
        color.to_class(),
        class.unwrap_or_default()
    );

    rsx! {
        div { class: "flex items-center",
            div { class: "{spinner_class}", role: "status",
                span { class: "sr-only", "Loading..." }
            }
            {label.map(|text| rsx! {
                span { class: "ml-2", "{text}" }
            })}
        }
    }
}