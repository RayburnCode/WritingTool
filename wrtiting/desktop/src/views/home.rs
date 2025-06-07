use std::ops::Add;

use dioxus::prelude::*;
use ui::Hero;
use crate::views::posts::{Posts, PostsContainer,AddPost};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        //  Echo {}

        h2 { class: "text-xl font-bold text-gray-800 my-4 p-2 bg-gray-100 rounded",
            "Use this area below for adding additional Posts!"
        }
        AddPost {
            on_post_added: move |post_id| {
                tracing::info!("Post added with ID: {}", post_id);
            },
        }
        Posts {}
        PostsContainer {}
    }
}
