#![allow(unused)]

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::db::user_queries;
use crate::jwtauth::Claims;
use crate::models::inputs::Preferences;
use crate::models::outputs::StatusResponse;

use firebase_auth::FirebaseUser;

pub async fn create_user(req: HttpRequest) -> impl Responder {
    // Try to get FirebaseUser from request extensions (set by middleware)
    let user = match req.extensions().get::<FirebaseUser>().cloned() {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized().json(StatusResponse {
                status: "error".to_string(),
                message: Some("No Firebase authentication found".to_string()),
            });
        }
    };

    // Log user info
    println!("Firebase User ID: {}", user.user_id);
    println!("Firebase Email: {:?}", user.email);
    println!("Firebase Name: {:?}", user.name);
    // println!("Firebase Phone: {:?}", user.phone);
    // println!("Firebase Photo URL: {:?}", user.photo_url);

    
    HttpResponse::Ok().json(StatusResponse {
        status: "success".to_string(),
        message: Some(format!("User {} successfully logged in", user.user_id))
    })
}

pub async fn update_user_preference(pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<Preferences>) -> impl Responder {
    let Some(claims) = req.extensions().get::<Claims>().cloned() else {
        return HttpResponse::Unauthorized().json(StatusResponse {
            status: "error".to_string(),
            message: Some("No authentication claims found".to_string()),
        });
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
            });
        }
    };

    // Convert Preferences struct to JSON for storage
    let preferences_json = json!({
        "ageRange": body.age_range.as_ref().map(|r| json!({"min": r.min, "max": r.max})),
        "distanceMax": body.distance_max,
        "genderPreference": body.gender_preference,
        "ethnicityPreference": body.ethnicity_preference,
        "religionPreference": body.religion_preference
    });

    match user_queries::update_user_preferences(&pool, &user_id, preferences_json).await {
        Ok(_) => HttpResponse::Ok().json(StatusResponse {
            status: "success".to_string(),
            message: Some("User preferences updated successfully".to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to update user preferences: {}", e)),
        }),
    }
}