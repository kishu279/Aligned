use crate::models::inputs::UpdateProfileRequest;
use actix_web::{HttpResponse, Responder, web};

pub async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("Profile: Get Current Profile")
}

pub async fn update_profile(body: web::Json<UpdateProfileRequest>) -> impl Responder {
    println!("Profile: Update Profile - Name: {:?}", body.name);
    HttpResponse::Ok().body("Profile: Update Profile")
}

pub async fn upload_profile_images() -> impl Responder {
    // Multipart handling would go here
    HttpResponse::Ok().body("Profile: Upload Images")
}

pub async fn finalize_profile() -> impl Responder {
    HttpResponse::Ok().body("Profile: Finalize (Go Live)")
}

pub async fn delete_account() -> impl Responder {
    HttpResponse::Ok().body("Profile: Delete Account")
}
