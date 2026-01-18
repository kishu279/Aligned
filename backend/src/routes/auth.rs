// ============================================================
// DEPRECATED: This auth module is not currently in use.
// Firebase handles phone authentication on the frontend.
// The user is created via POST /api/v1/user/create with Firebase token.
// ============================================================

#![allow(unused)]

use crate::models::inputs::{PhoneLoginRequest, PhoneVerifyRequest};
use crate::models::outputs::{AuthResponse, LoginResponse, StatusResponse, UserSummary};
use crate::models::state::AppState;
use crate::jwtauth::Claims;
use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use crate::db;

pub async fn phone_login(body: web::Json<PhoneLoginRequest>, _pool: web::Data<PgPool>, state: web::Data<AppState>) -> impl Responder {
    println!("Auth: Phone Login for {}", body.phone);
    
    let verification_id = uuid::Uuid::new_v4().to_string();
    
    // Store verification_id -> phone in app state
    state.pending_verifications.lock().unwrap().insert(verification_id.clone(), body.phone.clone());
    
    // Return JSON response
    HttpResponse::Ok().json(LoginResponse {
        message: String::from("Verification code sent successfully"),
        verification_id,
    })
}

/*
// DEPRECATED: This function used get_or_create_user_by_phone which is no longer available
// Since both email and phone are now required, this flow doesn't work.
// Use Firebase auth + POST /api/v1/user/create instead.

pub async fn phone_verify(body: web::Json<PhoneVerifyRequest>, pool: web::Data<PgPool>, state: web::Data<AppState>) -> impl Responder {
    println!(
        "Auth: Verify Code {} for ID {}",
        body.code, body.verification_id
    );

    // get the verification state
    let phone = state.pending_verifications.lock().unwrap().get(&body.verification_id).cloned();

    let phone = match phone {
        Some(p) => p,
        None => return HttpResponse::NotFound().json(StatusResponse {
            status: "error".to_string(),
            message: Some("Invalid verification ID".to_string()),
        }),
    };

    // check the code
    if body.code != "123456" {
        return HttpResponse::Unauthorized().json(StatusResponse {
            status: "error".to_string(),
            message: Some("Invalid verification code".to_string()),
        });
    }
    
    // Step 1: Get or create the user in the database
    // DEPRECATED: This function requires only phone, but now both email and phone are required
    let (user_id, is_new_user) = match db::user_queries::get_or_create_user_by_phone(&pool, &phone).await {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to get/create user: {:?}", e);
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
            });
        }
    };

    // Step 2: Create token with user_id as subject
    let my_claim = Claims {
        sub: user_id.clone(),
        company: "Aligned".to_string(),
        exp: (Utc::now().timestamp() + 86400) as usize,
    };

    // Step 3: Generate token and return response
    match Claims::create_new_token(&my_claim) {
        Ok(token) => {
            HttpResponse::Ok().json(AuthResponse {
                token,
                user: UserSummary {
                    id: user_id,
                    is_profile_complete: false,
                    is_new_user,
                },
            })
        }
        Err(_err) => {
            HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Internal server error".to_string()),
            })
        }
    }
}
*/
