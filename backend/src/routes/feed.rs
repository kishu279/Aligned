use actix_web::{HttpResponse, Responder};

pub async fn get_feed() -> impl Responder {
    HttpResponse::Ok().body("Feed: Get Recommendations")
}
