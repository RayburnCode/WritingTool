//pg_app/server/src/lib.rs
pub mod post_functions;  // Contains server logic (e.g., handling requests, etc.)
pub use post_functions::{
    create_post, get_all_posts, find_post, update_post, delete_post, get_post_count
};

pub mod post_model;
pub use post_model::*;