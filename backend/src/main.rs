#![allow(unused)]

// use crate::jwtauth::Claims;
use actix_web::{App, HttpResponse, HttpServer, Responder, web, middleware::Logger};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_cors::Cors;
use actix_web::http::header;
use sqlx::postgres::PgPoolOptions;
// use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use std::time::Duration;

use firebase_auth::{FirebaseAuth, FirebaseUser};

mod db;
mod jwtauth;
mod models;
mod routes;
mod firebaseauth;

use routes::{auth, feed, interactions, matches, profile, prompts, user};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I'm ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv::dotenv().ok();

    // get the db url
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    // Firebase Project ID (NOT the Google OAuth Client ID!)
    // You can find this in Firebase Console > Project Settings, or in the JWT token's "aud" claim
    let project_id = env::var("PROJECT_ID").unwrap_or_else(|_| panic!("PROJECT_ID MUST BE SET"));

    // pg connection to connect to the pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url.as_str())
        .await
        .expect("Failed to connect to database");

    // Create AppState BEFORE the closure so it's shared across all workers
    let app_state = web::Data::new(models::state::AppState {
        pending_verifications: Mutex::new(HashMap::new()),
    });

    let firebase_auth = FirebaseAuth::new(&project_id).await;
    let app_firebase = web::Data::new(firebase_auth);

    println!("Starting server on 0.0.0.0:8080 (accessible from network)");
    HttpServer::new(move || {

        // Create the auth middleware
        // let auth = HttpAuthentication::bearer(Claims::jwt_validator);

        App::new()
        .app_data(app_firebase.clone())
        .wrap(Cors::permissive())
        .route("/test", web::get().to(health_check))
        .route("/health", web::get().to(health_check))
        
        // Protected routes (auth required) - wrapped in a scope with middleware
        .service(
            web::scope("/api/v1")
            .wrap(Cors::permissive())
            // .wrap(Cors::default()
            //     .allowed_origin("**")
            //     .allow_any_origin()
            //     .allow_any_header()
            //     .allow_any_method()
            //     // .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"])
            //     // .allowed_headers(vec!["Authorization", "Content-Type"])
            //     // // .allowed_headers(http::header::CONTENT_TYPE)
            //     // .expose_headers(vec!["Authorization"])
            //     // // .supports_credentials()
            // )
            // .app_data(web::Data::new(pool.clone()))
            // .app_data(app_state.clone())
            // .app_data(app_firebase.clone())
            .wrap(firebaseauth::middleware::FirebaseAuthMiddleware)
            .route("/test", web::get().to(health_check))  // Test route in /api/v1 scope
            .route("/user/create", web::post().to(user::create_user))
            // .wrap(auth)
            .route("/profile/me", web::get().to(profile::get_profile))
            .route("/profile", web::post().to(profile::update_profile))
            .route("/user/preferences", web::post().to(user::update_user_preference))
            .route("/user/images", web::post().to(profile::upload_user_images))
            .route("/profile/images", web::post().to(profile::upload_profile_images))
            .route("/profile/finalize", web::post().to(profile::finalize_profile))
            .route("/profile", web::delete().to(profile::delete_account))
            .route("/feed", web::get().to(feed::get_feed))
            .route("/interact", web::post().to(interactions::interact))
            .route("/matches", web::get().to(matches::get_matches))
            .route(
                "/matches/{id}/messages",
                    web::get().to(matches::get_messages),
                )
            .route(
                    "/matches/{id}/messages",
                    web::post().to(matches::send_message),
                )
            // Prompts routes
            .route("/prompts", web::get().to(prompts::get_prompts))
                .route("/prompts", web::post().to(prompts::create_prompt))
                .route("/prompts/{order}", web::put().to(prompts::update_prompt))
        .route("/prompts/{order}", web::delete().to(prompts::delete_prompt)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

/*
Route Descriptions:

GET /health
- Health check for load balancers.

POST /auth/phone/login
- Initiates phone authentication (sends OTP).

POST /auth/phone/verify
- Verifies OTP and returns auth token + user info.

GET /profile/me
- Gets the current authenticated user's profile details.

POST /profile
- Updates profile fields (name, bio, etc.).

POST /profile/images
- Uploads a user profile image.

POST /profile/finalize
- Finalizes profile (sets "is_profile_complete") after ensuring 6 images are present.

DELETE /profile
- Deletes the user account permanently.

GET /feed
- Gets recommended profiles for the user to swipe on.

POST /interact
- Handles Like (Heart) or Pass (Cross) interactions.

GET /matches
- Gets a list of all matches (conversations).

GET /matches/{id}/messages
- Gets the chat history for a specific match.

POST /matches/{id}/messages
- Sends a new message to a match.
*/
