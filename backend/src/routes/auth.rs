use crate::models::inputs::{PhoneLoginRequest, PhoneVerifyRequest};
use actix_web::{HttpResponse, Responder, web};
// use crate::models::outputs::{LoginResponse, AuthResponse}; // Use these later when implementing logic

pub async fn phone_login(body: web::Json<PhoneLoginRequest>) -> impl Responder {
    println!("Auth: Phone Login for {}", body.phone);
    HttpResponse::Ok().body("Auth: Phone Login Endpoint")
}

pub async fn phone_verify(body: web::Json<PhoneVerifyRequest>) -> impl Responder {
    println!(
        "Auth: Verify Code {} for ID {}",
        body.code, body.verification_id
    );
    HttpResponse::Ok().body("Auth: Phone Verify Endpoint")
}
