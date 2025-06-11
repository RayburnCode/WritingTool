//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}


mod ollama;
pub use ollama::send_prompt_to_ollama;

pub mod posts;
pub use posts::{
    create_post, get_all_posts, find_post, update_post, delete_post,
    Post,
};

pub mod db;
pub use db::{get_db, init_db};

pub mod authentication;


pub mod users;

pub mod role;

pub mod session;

pub mod comment;