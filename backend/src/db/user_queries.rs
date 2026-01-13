use sqlx::PgPool;

/// Check if the user exists in the database by phone
/// Returns Some(id) if user exists, None if not found
pub async fn check_user_exists(pool: &PgPool, phone: &str) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id FROM users WHERE phone = $1",
        phone,
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.id.to_string()))
}

/// Create a new user in the database
/// Returns the new user's id
pub async fn create_user(pool: &PgPool, phone: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query!(
        "INSERT INTO users (phone) VALUES ($1) RETURNING id",
        phone,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id.to_string())
}

/// Get or create a user by phone number
/// Returns (user_id, is_new_user)
pub async fn get_or_create_user(pool: &PgPool, phone: &str) -> Result<(String, bool), sqlx::Error> {
    // First check if user exists
    if let Some(id) = check_user_exists(pool, phone).await? {
        return Ok((id, false));
    }
    
    // Create new user
    let id = create_user(pool, phone).await?;
    Ok((id, true))
}