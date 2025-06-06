use dioxus::prelude::*;
use ui::Hero;
use crate::views::posts::{Posts, PostsContainer};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
      //  Echo {}

        h2 { 
            class: "text-xl font-bold text-gray-800 my-4 p-2 bg-gray-100 rounded",
            "Use this area below for adding additional Posts!" 
        }        
        Posts {}
        PostsContainer{}
    }
}
