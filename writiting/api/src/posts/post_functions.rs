// pg_app/server/src/server_functions.rs
use dioxus::prelude::*;
use crate::Post;
use crate::ServerFnError;
use crate::db::connection_pool::get_db; 
use tracing::info;


#[server]
pub async fn create_post(title: String, body: String) -> Result<i32, ServerFnError> {
    let db = get_db().await;
    
    // Input validation
    if title.trim().is_empty() || body.trim().is_empty() {
        return Err(ServerFnError::Request("Title and body cannot be empty".into()));
    }

    match sqlx::query!(
        "INSERT INTO posts (title, body) VALUES ($1, $2) RETURNING id",
        title.trim(),
        body.trim()
    )
    .fetch_one(db)
    .await
    {
        Ok(record) => {
            info!("Created new post with ID: {}", record.id);
            Ok(record.id)
        },
        Err(e) => {
            tracing::error!("Database error creating post: {}", e);
            Err(ServerFnError::ServerError("Failed to create post".into()))
        }
    }
}



#[server]
pub async fn get_all_posts() -> Result<Vec<Post>, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(db)
        .await?;

    Ok(result)
}


// In your API module (api/posts.rs)
pub async fn find_post(id: i32) -> Result<Post, ServerFnError> {
    let db = get_db().await?;

    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, body, created_at, updated_at
        FROM posts
        WHERE id = $1
        "#,
        id  // Make sure this parameter is included
    )
    .fetch_one(db)  // Use your database pool here
    .await?;

    Ok(post)
}


#[server]
pub async fn delete_post(id: i32) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(db)
        .await?;

    match result.rows_affected() {
        0 => Err(ServerFnError::Request("No rows deleted".to_string())),
        _ => Ok(()),
    }
}



#[server]
pub async fn update_post(id: i32, title: String, body: String) -> Result<Post, ServerFnError> {
    let db = get_db().await;
    
    if title.is_empty() || body.is_empty() {
        return Err(ServerFnError::Request("Title and body cannot be empty".into()));
    }
    
    sqlx::query_as::<_, Post>(
        "UPDATE posts SET title = $1, body = $2 WHERE id = $3 RETURNING *"
    )
    .bind(title.trim())
    .bind(body.trim())
    .bind(id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update post: {}", e);
        ServerFnError::ServerError("Failed to update post".into())
    })
}


#[server]
pub async fn get_post_count() -> Result<i32, ServerFnError> {
    let db = get_db().await;
    let count = sqlx::query!("SELECT COUNT(*) as count FROM posts")
        .fetch_one(db)
        .await?
        .count // Returns 0 if no posts exist
        .map(|v| v as i32);  // Convert Some(i64) to Some(i32)

    Ok(count.expect("REASON"))
}