#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// The source URL of the avatar image (optional)
    src: Option<String>,
    /// Alternative text for the avatar image
    alt: Option<String>,
    /// Additional CSS classes for the avatar container
    class: Option<String>,
    /// Additional CSS classes for the image itself
    image_class: Option<String>,
    /// Size variant of the avatar (small, medium, large)
    size: Option<AvatarSize>,
    /// Whether the avatar should be circular (default) or square
    rounded: Option<bool>,
    /// Optional click handler
    onclick: Option<EventHandler<MouseEvent>>,
    /// Optional href to make the avatar a link
    href: Option<String>,
    /// Initials to display when no src is provided
    initials: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    // Determine size classes
    let size_class = match props.size {
        Some(AvatarSize::Small) => "w-8 h-8",
        Some(AvatarSize::Medium) => "w-12 h-12",
        Some(AvatarSize::Large) => "w-16 h-16",
        None => "w-10 h-10", // default size
    };

    // Determine rounded style
    let rounded_class = if props.rounded.unwrap_or(true) {
        "rounded-full"
    } else {
        "rounded"
    };

    // Combine all classes
    let container_class = format!(
        "{} {} {}",
        size_class,
        rounded_class,
        props.class.clone().unwrap_or_default()
    );

    let image_class = format!(
        "object-cover {}",
        props.image_class.clone().unwrap_or_default()
    );

    // Get initials (default to "DR" if not provided)
    let initials = props.initials.clone().unwrap_or("DR".to_string());
    
    // Render the content based on whether src is provided
    let content = {
        if let Some(src) = &props.src {
            // If src is provided, render the image
            rsx! {
                img {
                    class: "{image_class} {container_class}",
                    src: "{src}",
                    alt: props.alt.clone().unwrap_or("Avatar".to_string()),
                }
            }
        } else {
            // If no src is provided, render a blue circle with initials
            rsx! {
                div { class: "{container_class} bg-blue-100 flex items-center justify-center text-white font-medium",
                    "{initials}"
                }
            }
        }
    };

    // If href is provided, wrap in an <a> tag, otherwise just render the content
    if let Some(href) = &props.href {
        rsx! {
            a {
                href: "{href}",
                class: "inline-block",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {content}
            }
        }
    } else {
        rsx! {
            div {
                class: "inline-block",
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {content}
            }
        }
    }
}

// // Basic usage with default size (w-10 h-10)
// Avatar {
//     src: Some("/path/to/avatar.jpg".to_string()),
//     alt: Some("User profile".to_string()),
// }
//
// // Avatar with no src (will display initials)
// Avatar {
//     src: None,
//     initials: Some("JD".to_string()),
// }
//
// // With custom size
// Avatar {
//     src: Some("/path/to/avatar.jpg".to_string()),
//     size: Some(AvatarSize::Large),
//     alt: Some("User profile".to_string()),
// }
//
// // Square avatar instead of rounded
// Avatar {
//     src: Some("/path/to/avatar.jpg".to_string()),
//     rounded: Some(false),
//     alt: Some("User profile".to_string()),
// }
//
// // As a link
// Avatar {
//     src: Some("/path/to/avatar.jpg".to_string()),
//     href: Some("/user/profile".to_string()),
//     alt: Some("User profile".to_string()),
// }
//
// // With additional classes
// Avatar {
//     src: Some("/path/to/avatar.jpg".to_string()),
//     class: Some("border-2 border-white".to_string()),
//     image_class: Some("hover:opacity-90".to_string()),
//     alt: Some("User profile".to_string()),
// }