use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{profile_queries, user_queries};
use crate::jwtauth::Claims;
use crate::models::inputs::Preferences;
use crate::models::outputs::{FeedResponse, ProfileDetails, StatusResponse, UserProfile};

pub async fn get_feed(pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
    let Some(claim) = req.extensions().get::<Claims>().cloned() else {
        return HttpResponse::Unauthorized().json(StatusResponse {
            status: "error".to_string(),
            message: Some("No authentication claims found".to_string()),
        });
    };

    let Ok(user_id) = Uuid::parse_str(&claim.sub) else {
        return HttpResponse::Unauthorized().json(StatusResponse {
            status: "error".to_string(),
            message: Some("Invalid user ID format".to_string()),
        });
    };

    // Get user's preferences (from users table - JSONB field)
    let preferences = match user_queries::get_user_preferences(&pool, &user_id).await {
        Ok(prefs) => prefs,
        Err(e) => {
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to get preferences: {}", e)),
            });
        }
    };

    // Check if preferences exist
    let Some(prefs_json) = preferences else {
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
