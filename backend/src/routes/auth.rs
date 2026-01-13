use crate::models::inputs::{PhoneLoginRequest, PhoneVerifyRequest};
use crate::models::outputs::{AuthResponse, LoginResponse, StatusResponse, UserSummary};
use crate::models::state::AppState;
use crate::jwtauth::Claims;
use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;

pub async fn phone_login(body: web::Json<PhoneLoginRequest>, _pool: web::Data<PgPool>, state: web::Data<AppState>) -> impl Responder {
    println!("Auth: Phone Login for {}", body.phone);
    
    let verification_id = uuid::Uuid::new_v4().to_string();
    // let verification_code = "123456"; // TODO: Generate a random code
    
    // Store verification_id -> phone in app state
    state.pending_verifications.lock().unwrap().insert(verification_id.clone(), body.phone.clone());
    
    // Return JSON response
    HttpResponse::Ok().json(LoginResponse {
        message: String::from("Verification code sent successfully"),
        verification_id,
    })
}

pub async fn phone_verify(body: web::Json<PhoneVerifyRequest>, _pool: web::Data<PgPool>, state: web::Data<AppState>) -> impl Responder {
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

    // create the token
    let my_claim = Claims {
        sub: phone.clone().to_string(),
        company: "Aligned".to_string(),
        exp: (Utc::now().timestamp() + 86400) as usize,
    };
    
    match Claims::create_new_token(my_claim) {
        Ok(token) => {
            // TODO: create the db entry of the phone number
            // ..

            HttpResponse::Ok().json(AuthResponse {
                token,
                user: UserSummary {
                    id: phone.clone().to_string(),
                    is_profile_complete: false,
                    is_new_user: true,
                },
            })

        },
        Err(err) => {
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Internal server error".to_string()),
            });
        }
    }
}
