#![allow(unused)]

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::db::user_queries;
use crate::jwtauth::Claims;
use crate::models::inputs::Preferences;
use crate::models::outputs::StatusResponse;

use crate::models::inputs::CreateUserRequest;
use crate::models::inputs::CheckUserExistsRequest;

use firebase_auth::FirebaseUser;

pub async fn create_user(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateUserRequest>,
) -> impl Responder {
    // Try to get FirebaseUser from request extensions (set by middleware)
    let firebase_user = match req.extensions().get::<FirebaseUser>().cloned() {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized().json(StatusResponse {
                status: "error".to_string(),
                message: Some("No Firebase authentication found".to_string()),
            });
        }
    };

    // Log Firebase user info
    println!("Firebase User ID: {}", firebase_user.user_id);
    println!("Firebase Email: {:?}", firebase_user.email);
    println!("Request Body - Phone: {:?}, Email: {:?}", body.phone, body.email);

    // Validate that both phone and email are provided (required fields)
    let phone = match &body.phone {
        Some(p) if !p.is_empty() => p.as_str(),
        _ => {
            return HttpResponse::BadRequest().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Phone number is required".to_string()),
            });
        }
    };

    let email = match &body.email {
        Some(e) if !e.is_empty() => e.as_str(),
        _ => {
            return HttpResponse::BadRequest().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Email is required".to_string()),
            });
        }
    };

    // Check if user already exists by email or phone
    if let Ok(Some(existing_id)) = user_queries::check_user_exists(&pool, phone, email).await {
        return HttpResponse::Ok().json(StatusResponse {
            status: "success".to_string(),
            message: Some(format!("User {} already exists", existing_id)),
        });
    }

    // Create new user with both email and phone
    match user_queries::create_user(&pool, phone, email).await {
        Ok(user_id) => {
            println!("Created new user with ID: {}", user_id);
            HttpResponse::Ok().json(StatusResponse {
                status: "success".to_string(),
                message: Some(format!("User {} successfully created", user_id)),
            })
        }
        Err(e) => {
            eprintln!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to create user: {}", e)),
            })
        }
    }
}

pub async fn update_user_preference(pool: web::Data<PgPool>, _req: HttpRequest, body: web::Json<Preferences>) -> impl Responder {

    // Convert Preferences struct to JSON for storage
    let preferences_json = json!({
        "ageRange": body.age_range.as_ref().map(|r| json!({"min": r.min, "max": r.max})),
        "distanceMax": body.distance_max,
        "genderPreference": body.gender_preference,
        "ethnicityPreference": body.ethnicity_preference,
        "religionPreference": body.religion_preference
    });

    println!("User {:?}", preferences_json);

    // Update preferences - will find user by email, then phone
    match user_queries::update_user_preferences(
        &pool, 
        None,  // No user_id from JWT - find by email/phone
        body.email.as_deref(), 
        body.phone.as_deref(), 
        preferences_json
    ).await {
        Ok(_) => HttpResponse::Ok().json(StatusResponse {
            status: "success".to_string(),
            message: Some("User preferences updated successfully".to_string()),
        }),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(StatusResponse {
            status: "error".to_string(),
            message: Some("User not found".to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to update user preferences: {}", e)),
        }),
    }
}

pub async fn check_user_exists(pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<CheckUserExistsRequest>) -> impl Responder {
    
    // check for the user is present on the table or not
    match user_queries::check_user_exists_optional(&pool, body.phone.clone(), body.email.clone()).await {
        Ok(Some(user_id)) => HttpResponse::Ok().json(StatusResponse {
            status: "exists".to_string(),
            message: Some(format!("User {} already exists", user_id)),
        }),
        Ok(None) => HttpResponse::Ok().json(StatusResponse {
            status: "not_found".to_string(),
            message: Some("User does not exist".to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to check user exists: {}", e)),
        }),
    } 
}

pub async fn get_user(pool: web::Data<PgPool>, body: web::Json<CheckUserExistsRequest>) -> impl Responder {
    // Check if at least one identifier is provided
    let (phone, email) = (body.phone.clone(), body.email.clone());
    
    if phone.is_none() && email.is_none() {
        return HttpResponse::BadRequest().json(StatusResponse {
            status: "error".to_string(),
            message: Some("At least one of phone or email is required".to_string()),
        });
    }

    // Try to find user by phone first, then email
    let user_result = match (&phone, &email) {
        (Some(p), _) => user_queries::check_user_exists_by_phone(&pool, p).await,
        (None, Some(e)) => user_queries::check_user_exists_by_email(&pool, e).await,
        _ => unreachable!(), // Already handled above
    };

    match user_result {
        Ok(Some(user_id)) => {
            // Found user ID, now fetch full user details
            let uuid = match Uuid::parse_str(&user_id) {
                Ok(id) => id,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(StatusResponse {
                        status: "error".to_string(),
                        message: Some("Invalid user ID in database".to_string()),
                    });
                }
            };

            match user_queries::get_user(&pool, &uuid).await {
                Ok(Some(user)) => HttpResponse::Ok().json(json!({
                    "status": "success",
                    "user": {
                        "id": user.id.to_string(),
                        "phone": user.phone,
                        "email": user.email,
                        "preferences": user.preferences
                    }
                })),
                Ok(None) => HttpResponse::NotFound().json(StatusResponse {
                    status: "not_found".to_string(),
                    message: Some("User not found".to_string()),
                }),
                Err(e) => HttpResponse::InternalServerError().json(StatusResponse {
                    status: "error".to_string(),
                    message: Some(format!("Failed to get user: {}", e)),
                }),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(StatusResponse {
            status: "not_found".to_string(),
            message: Some("User not found".to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to find user: {}", e)),
        }),
    }
}