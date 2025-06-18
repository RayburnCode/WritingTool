// pg_app/server/src/server_functions.rs
use dioxus::prelude::*;
use crate::users::{UserProfile, ProfileUpdate, User};
use crate::ServerFnError;
use crate::db::connection_pool::get_db;
use uuid::Uuid;
use tracing::info;

#[server]
pub async fn create_usesr(user_id: Uuid) -> Result<(), ServerFnError> {
    let db = get_db().await;
    
    sqlx::query!(
        "INSERT INTO users (user_id) VALUES ($1)",
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error creating user: {}", e);
        ServerFnError::ServerError("Failed to create user".into())
    })?;

    info!("Created user: {}", user_id);
    Ok(())
} 

#[server]
pub async fn get_user(user_id: Uuid) -> Result<User, ServerFnError> {
    let db = get_db().await;

    sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE id = $1
        "#
    )
    .bind(user_id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error fetching user: {}", e);
        ServerFnError::ServerError("User not found".into())
    })
}

#[server]
pub async fn get_user_with_profile(user_id: Uuid) -> Result<(User, UserProfile), ServerFnError> {
    let db = get_db().await;

    let user = get_user(user_id).await?;
    let profile = get_profile(user_id).await?;

    Ok((user, profile))
}

#[server]
pub async fn update_user(
    user_id: Uuid,
    update: UserUpdate
) -> Result<User, ServerFnError> {
    let db = get_db().await;
    
    // Validate input
    update.validate().map_err(|e| {
        tracing::error!("Validation error: {:?}", e);
        ServerFnError::Request("Invalid user data".into())
    })?;

    sqlx::query_as::<_, User>(
        r#"
        UPDATE users SET
            email = COALESCE($1, email),
            username = COALESCE($2, username),
            role_id = COALESCE($3, role_id),
            is_active = COALESCE($4, is_active),
            email_verified = COALESCE($5, email_verified)
        WHERE id = $6
        RETURNING *
        "#
    )
    .bind(update.email)
    .bind(update.username)
    .bind(update.role_id)
    .bind(update.is_active)
    .bind(update.email_verified)
    .bind(user_id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error updating user: {}", e);
        ServerFnError::ServerError("Failed to update user".into())
    })
}

#[server]
pub async fn delete_user(user_id: Uuid) -> Result<(), ServerFnError> {
    let db = get_db().await;

    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error deleting user: {}", e);
        ServerFnError::ServerError("Failed to delete user".into())
    })?;

    info!("Deleted user: {}", user_id);
    Ok(())
}

#[server]
pub async fn get_profile_with_user(user_id: Uuid) -> Result<(UserProfile, User), ServerFnError> {
    let db = get_db().await;

    let profile = sqlx::query_as::<_, UserProfile>(
        "SELECT * FROM profiles WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    Ok((profile, user))
}

