

pub mod add_post;  // Contains AddPost
pub mod all_post;      // Contains Post
pub mod update_post;
pub mod update_by_id;
pub mod display_post;

pub use add_post::AddPost;
pub use all_post::DisplayAllPosts;
pub use update_post::UpdatePost;
pub use update_by_id::UpdatePostsById;
pub use display_post::DisplayPostById;
