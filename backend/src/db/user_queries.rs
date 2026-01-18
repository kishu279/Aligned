use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// User struct for database queries
#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub preferences: Option<serde_json::Value>,
}

/// Check if the user exists in the database by phone or email
/// Returns Some(id) if user exists, None if not found
pub async fn check_user_exists_by_phone(pool: &PgPool, phone: &str) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM users WHERE phone = $1"
    )
    .bind(phone)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0.to_string()))
}

/// Check if the user exists in the database by email
/// Returns Some(id) if user exists, None if not found
pub async fn check_user_exists_by_email(pool: &PgPool, email: &str) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0.to_string()))
}

/// Check if the user exists by phone OR email
/// Returns Some(id) if user exists, None if not found
pub async fn check_user_exists(pool: &PgPool, phone: &str, email: &str) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM users WHERE phone = $1 OR email = $2"
    )
    .bind(phone)
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0.to_string()))
}

/// Check if the user exists by optional phone OR email
/// Returns Some(id) if user exists, None if not found
pub async fn check_user_exists_optional(pool: &PgPool, phone: Option<String>, email: Option<String>) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM users WHERE ($1::text IS NULL OR phone = $1) AND ($2::text IS NULL OR email = $2) AND (phone IS NOT NULL OR email IS NOT NULL)"
    )
    .bind(&phone)
    .bind(&email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0.to_string()))
}

/// Create a new user in the database with BOTH phone and email (required)
/// Returns the new user's id
pub async fn create_user(pool: &PgPool, phone: &str, email: &str) -> Result<String, sqlx::Error> {
    let row: (Uuid,) = sqlx::query_as(
        "INSERT INTO users (phone, email) VALUES ($1, $2) RETURNING id"
    )
    .bind(phone)
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(row.0.to_string())
}

/// Get or create a user with both phone and email (required)
/// Returns (user_id, is_new_user)
pub async fn get_or_create_user(pool: &PgPool, phone: &str, email: &str) -> Result<(String, bool), sqlx::Error> {
    // First check if user exists by phone OR email
    if let Some(id) = check_user_exists(pool, phone, email).await? {
        return Ok((id, false));
    }
    
    // Create new user with both phone and email
    let id = create_user(pool, phone, email).await?;
    Ok((id, true))
}

// ============================================================
// DEPRECATED: Functions below are for single-field user creation
// Not needed when both email and phone are compulsory
// ============================================================

/*
/// Get or create a user by phone number only
/// Returns (user_id, is_new_user)
pub async fn get_or_create_user_by_phone(pool: &PgPool, phone: &str) -> Result<(String, bool), sqlx::Error> {
    // First check if user exists
    if let Some(id) = check_user_exists_by_phone(pool, phone).await? {
        return Ok((id, false));
    }
    
    // Create new user with phone only - NOT VALID when both fields required
    // let id = create_user(pool, phone, "").await?;
    // Ok((id, true))
    Err(sqlx::Error::Protocol("Both phone and email are required".to_string()))
}

/// Get or create a user by email only
/// Returns (user_id, is_new_user)
pub async fn get_or_create_user_by_email(pool: &PgPool, email: &str) -> Result<(String, bool), sqlx::Error> {
    // First check if user exists
    if let Some(id) = check_user_exists_by_email(pool, email).await? {
        return Ok((id, false));
    }
    
    // Create new user with email only - NOT VALID when both fields required
    // let id = create_user(pool, "", email).await?;
    // Ok((id, true))
    Err(sqlx::Error::Protocol("Both phone and email are required".to_string()))
}

/// Update user email
pub async fn update_user_email(pool: &PgPool, user_id: &Uuid, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET email = $2 WHERE id = $1"
    )
    .bind(user_id)
    .bind(email)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update user phone
pub async fn update_user_phone(pool: &PgPool, user_id: &Uuid, phone: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET phone = $2 WHERE id = $1"
    )
    .bind(user_id)
    .bind(phone)
    .execute(pool)
    .await?;

    Ok(())
}
*/

/// Get user by ID
pub async fn get_user(pool: &PgPool, user_id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    let user: Option<User> = sqlx::query_as(
        "SELECT id, phone, email, preferences FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Get user preferences as JSON
pub async fn get_user_preferences(pool: &PgPool, user_id: &Uuid) -> Result<Option<serde_json::Value>, sqlx::Error> {
    let row: Option<(serde_json::Value,)> = sqlx::query_as(
        "SELECT preferences FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0))
}

/// Update user preferences
pub async fn update_user_preferences(pool: &PgPool, user_id: &Uuid, preferences: serde_json::Value) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET preferences = $2 WHERE id = $1"
    )
    .bind(user_id)
    .bind(preferences)
    .execute(pool)
    .await?;

    Ok(())
}