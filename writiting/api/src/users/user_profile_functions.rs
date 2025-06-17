// pg_app/server/src/server_functions.rs
use dioxus::prelude::*;
use crate::users::{UserProfile, ProfileUpdate, User};
use crate::ServerFnError;
use crate::db::connection_pool::get_db;
use tracing::info;
use uuid::Uuid;

#[server]
pub async fn create_profile(user_id: Uuid) -> Result<(), ServerFnError> {
    let db = get_db().await;
    
    sqlx::query!(
        "INSERT INTO profiles (user_id) VALUES ($1)",
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error creating profile: {}", e);
        ServerFnError::ServerError("Failed to create profile".into())
    })?;

    info!("Created profile for user: {}", user_id);
    Ok(())
}

#[server]
pub async fn get_profile(user_id: Uuid) -> Result<UserProfile, ServerFnError> {
    let db = get_db().await;

    sqlx::query_as::<_, UserProfile>(
        r#"
        SELECT p.*, 
               u.id as "user_id", u.email, u.username, u.role_id,
               u.is_active, u.email_verified, u.created_at as "user_created_at",
               u.updated_at as "user_updated_at"
        FROM profiles p
        JOIN users u ON p.user_id = u.id
        WHERE p.user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error fetching profile: {}", e);
        ServerFnError::ServerError("Profile not found".into())
    })
}

#[server]
pub async fn update_profile(
    user_id: Uuid,
    update: ProfileUpdate
) -> Result<UserProfile, ServerFnError> {
    let db = get_db().await;
    
    // Validate input
    update.validate().map_err(|e| {
        tracing::error!("Validation error: {:?}", e);
        ServerFnError::Request("Invalid profile data".into())
    })?;

    sqlx::query_as::<_, UserProfile>(
        r#"
        UPDATE profiles SET
            first_name = COALESCE($1, first_name),
            last_name = COALESCE($2, last_name),
            bio = COALESCE($3, bio),
            avatar_url = COALESCE($4, avatar_url),
            website_url = COALESCE($5, website_url),
            updated_at = NOW()
        WHERE user_id = $6
        RETURNING *
        "#
    )
    .bind(update.first_name)
    .bind(update.last_name)
    .bind(update.bio)
    .bind(update.avatar_url)
    .bind(update.website_url)
    .bind(user_id)
    .fetch_one(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error updating profile: {}", e);
        ServerFnError::ServerError("Failed to update profile".into())
    })
}

#[server]
pub async fn delete_profile(user_id: Uuid) -> Result<(), ServerFnError> {
    let db = get_db().await;

    sqlx::query!(
        "DELETE FROM profiles WHERE user_id = $1",
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error deleting profile: {}", e);
        ServerFnError::ServerError("Failed to delete profile".into())
    })?;

    info!("Deleted profile for user: {}", user_id);
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


// // Creating a profile
// let create_profile = move |user_id| async move {
//     match create_profile(user_id).await {
//         Ok(_) => info!("Profile created"),
//         Err(e) => error!("Error: {:?}", e),
//     }
// };

// // Updating a profile
// let update = ProfileUpdate {
//     first_name: Some("John".into()),
//     last_name: Some("Doe".into()),
//     ..Default::default()
// };

// match update_profile(user_id, update).await {
//     Ok(profile) => println!("Updated: {:?}", profile),
//     Err(e) => println!("Error: {}", e),
// }