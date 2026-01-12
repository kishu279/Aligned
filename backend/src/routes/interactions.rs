use crate::models::inputs::InteractRequest;
use actix_web::{HttpResponse, Responder, web};

pub async fn interact(body: web::Json<InteractRequest>) -> impl Responder {
    println!(
        "Interaction: {} on user {}",
        body.action, body.target_user_id
    );
    HttpResponse::Ok().body("Interaction: Like/Pass")
}
