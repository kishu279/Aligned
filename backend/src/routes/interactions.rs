use crate::db::interact_queries;
use crate::models::inputs::InteractRequest;
use crate::models::outputs::StatusResponse;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize};

use firebase_auth::FirebaseUser;

pub async fn interact(body: web::Json<InteractRequest>, pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
    println!("invoked /interactions");
    
    // getting the userid 
    let user: FirebaseUser = match req.extensions().get::<FirebaseUser>().cloned() {
        Some(user) => user,
        // NOT POSSIBLE AS IS IS PASSED FROM THE MIDDLEWARE
        None => return HttpResponse::Unauthorized().json(StatusResponse {
            status: "error".to_string(),
            message: Some("Unauthorized".to_string()),
        })
    };

    // Get user ID from database using Firebase email
    let user_id = match crate::db::user_queries::get_user_id_by_email(&pool, user.email.as_deref()).await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().json(StatusResponse {
            status: "error".to_string(),
            message: Some("User not found".to_string()),
        }),
        Err(e) => return HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to get user: {}", e)),
        })
    };

    let Ok(target_user_id) = Uuid::parse_str(&body.target_user_id) else {
        return HttpResponse::BadRequest().json(StatusResponse {
            status: "error".to_string(),
            message: Some("Invalid target user ID".to_string()),
        })
    };

    match interact_queries::interact(&pool, &user_id, &target_user_id, &body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(StatusResponse {
            status: "success".to_string(),
            message: Some("Interaction recorded".to_string()),
        }),
        Err(_) => HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some("Failed to record interaction".to_string()),
        })
    }
}


#[derive(Deserialize)]
pub struct PathParams {
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub action: String,
}

// GET ALL THE INTEACTIONS WHICH ARE TO ME
pub async fn get_interactions_for_me(pool: web::Data<PgPool>, params: web::Path<PathParams>, query: web::Query<QueryParams>) -> impl Responder {
    let interactions = match interact_queries::get_interactions_to_user_id(&pool, &params.user_id, &query.action).await {
        Ok(interactions) => interactions,
        Err(e) => return HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to get interactions: {}", e)),
        })
    };

    HttpResponse::Ok().json(interactions)
}

// GET ALL THE INTEACTIONS WHICH ARE TO EVERYONE
pub async fn get_interactions_to_everyone(pool: web::Data<PgPool>, params: web::Path<PathParams>, query: web::Query<QueryParams>) -> impl Responder {
    let interactions = match interact_queries::get_interactions_from_user_id(&pool, &params.user_id, &query.action).await {
        Ok(interactions) => interactions,
        Err(e) => return HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to get interactions: {}", e)),
        })
    };

    HttpResponse::Ok().json(interactions)
}