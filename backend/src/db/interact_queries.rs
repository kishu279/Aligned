use sqlx::PgPool;
use uuid::Uuid;

use crate::models::inputs::InteractRequest;
use crate::models::outputs::Interaction;

pub async fn interact(
    pool: &PgPool,
    from_user_id: &Uuid,  
    to_user_id: &Uuid,
    body: &InteractRequest,
) -> Result<(), sqlx::Error> {
    // Extract context_type and context_id from the optional context
    let (context_type, context_id) = match &body.context {
        Some(ctx) => (Some(ctx.r#type.clone()), Some(ctx.id.clone())),
        None => (None, None),
    };

    sqlx::query(
        r#"INSERT INTO interactions (from_user_id, to_user_id, action, context_type, context_id, comment) 
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (from_user_id, to_user_id) 
           DO UPDATE SET action = $3, context_type = $4, context_id = $5, comment = $6"#
    )
    .bind(from_user_id)
    .bind(to_user_id)
    .bind(&body.action)
    .bind(&context_type)
    .bind(&context_id)
    .bind(&body.comment)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_interactions_to_user_id(
    pool: &PgPool,
    user_id: &Uuid,
    action: &str    
) -> Result<Vec<Interaction>, sqlx::Error> {
    let interactions = sqlx::query_as(
        r#"SELECT * FROM interactions WHERE to_user_id = $1 AND action = $2 ORDER BY created_at DESC"#
    )
    .bind(user_id)
    .bind(action)
    .fetch_all(pool)
    .await?;

    Ok(interactions)
}

pub async fn get_interactions_from_user_id(
    pool: &PgPool,
    user_id: &Uuid,
    action: &str,
) -> Result<Vec<Interaction>, sqlx::Error> {
    let interactions = sqlx::query_as(
        r#"SELECT * FROM interactions WHERE from_user_id = $1 AND action = $2 ORDER BY created_at DESC"#
    )
    .bind(user_id)
    .bind(action)
    .fetch_all(pool)
    .await?;

    Ok(interactions)
}