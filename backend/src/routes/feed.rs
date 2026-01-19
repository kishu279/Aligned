use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

use crate::db::{profile_queries, user_queries};
use crate::models::inputs::{Preferences, FeedRequest};
use crate::models::outputs::{FeedResponse, ProfileDetails, StatusResponse, UserProfile};

pub async fn get_feed(pool: web::Data<PgPool>, body: web::Json<FeedRequest>) -> impl Responder {
    println!("POST /feed invoked");

    // Find user by email or phone
    let (user_id, preferences_opt) = match user_queries::get_user_with_preferences_by_identifier(
        &pool,
        body.email.as_deref(),
        body.phone.as_deref(),
    ).await {
        Ok(Some((uid, prefs))) => (uid, prefs),
        Ok(None) => {
            return HttpResponse::NotFound().json(StatusResponse {
                status: "error".to_string(),
                message: Some("User not found".to_string()),
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to find user: {}", e)),
            });
        }
    };

    // Check if preferences exist
    let Some(prefs_json) = preferences_opt else {
        return HttpResponse::BadRequest().json(StatusResponse {
            status: "error".to_string(),
            message: Some("User has no preferences set".to_string()),
        });
    };

    // Parse the JSON into Preferences struct
    let preference: Preferences = match serde_json::from_value(prefs_json) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to parse preferences: {}", e)),
            });
        }
    };

    // Get suggestions based on gender preference only (for now)
    let suggestions =
        match profile_queries::get_suggestions(&pool, preference.gender_preference, &user_id).await
        {
            Ok(profiles) => profiles,
            Err(e) => {
                return HttpResponse::InternalServerError().json(StatusResponse {
                    status: "error".to_string(),
                    message: Some(format!("Failed to get suggestions: {}", e)),
                });
            }
        };

    // Convert SuggestionProfile to UserProfile for the response
    let profiles: Vec<UserProfile> = suggestions
        .into_iter()
        .map(|p| UserProfile {
            id: p.user_id.clone(),
            images: None,
            prompts: None,
            details: Some(ProfileDetails {
                name: p.name,
                bio: p.bio,
                birthdate: p.birthdate,
                pronouns: p.pronouns,
                gender: p.gender,
                sexuality: p.sexuality,
                height: p.height,
                location: p.location,
                job: p.job,
                company: p.company,
                school: p.school,
                ethnicity: p.ethnicity,
                politics: p.politics,
                religion: p.religion,
                relationship_type: p.relationship_type,
                dating_intention: p.dating_intention,
                drinks: p.drinks,
                smokes: p.smokes,
            }),
        })
        .collect();

    HttpResponse::Ok().json(FeedResponse { profiles })
}
