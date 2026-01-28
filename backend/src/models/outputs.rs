use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

// Generic Responses
#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub message: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub pending_actions: Option<Vec<String>>,
}

// Auth
#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub verification_id: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserSummary,
}

#[derive(Serialize)]
pub struct UserSummary {
    pub id: String,
    pub is_profile_complete: bool,
    pub is_new_user: bool,
}

// Profile
#[derive(Serialize, Debug)]
pub struct UserProfile {
    pub id: String,
    pub images: Option<Vec<UserImage>>,
    pub prompts: Option<Vec<UserPrompt>>,
    pub details: Option<ProfileDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserImage {
    #[serde(alias = "image_id")]
    pub id: String,
    pub url: String,
    pub order: i32,
}

#[derive(Serialize, Debug)]
pub struct UserPrompt {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub order: i32,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct ProfileDetails {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub birthdate: Option<String>, // DATE - "1995-03-15"
    pub pronouns: Option<String>,  // "she/her", "he/him"
    pub gender: Option<String>,    // "Woman", "Man", "Non-binary"
    pub sexuality: Option<String>, // "Straight", "Gay", "Bisexual"
    pub height: Option<i32>,       // in cm
    pub location: Option<String>,  // Will need to convert to POINT
    pub job: Option<String>,
    pub company: Option<String>,
    pub school: Option<String>,
    pub ethnicity: Option<String>,
    pub politics: Option<String>,
    pub religion: Option<String>,
    pub relationship_type: Option<String>, // "Monogamy"
    pub dating_intention: Option<String>,  // "Long-term relationship"
    pub drinks: Option<String>,            // "Socially", "No"
    pub smokes: Option<String>,
}

/// Profile with user_id for feed suggestions
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct SuggestionProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub birthdate: Option<String>,
    pub pronouns: Option<String>,
    pub gender: Option<String>,
    pub sexuality: Option<String>,
    pub height: Option<i32>,
    pub location: Option<String>,
    pub job: Option<String>,
    pub company: Option<String>,
    pub school: Option<String>,
    pub ethnicity: Option<String>,
    pub politics: Option<String>,
    pub religion: Option<String>,
    pub relationship_type: Option<String>,
    pub dating_intention: Option<String>,
    pub drinks: Option<String>,
    pub smokes: Option<String>,
    pub images: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct ImageUploadResponse {
    pub id: String,
    pub url: String,
    pub order: i32,
}

#[derive(Serialize)]
pub struct FinalizeProfileResponse {
    pub status: String,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<Vec<String>>,
}

// Feed
#[derive(Serialize)]
pub struct FeedResponse {
    pub profiles: Vec<UserProfile>,
}

// Interactions
#[derive(Serialize)]
pub struct MatchResponse {
    pub status: String, // "MATCH" or "SENT"
    pub match_id: Option<String>,
    pub match_data: Option<MatchData>,
}

#[derive(Serialize)]
pub struct MatchData {
    pub user: UserSummary, // Simplified for now
}

// Messages
#[derive(Serialize)]
pub struct MatchSummary {
    pub id: String,
    pub with_user: UserSummary,
    pub last_message: Option<MessagePreview>,
}

#[derive(Serialize)]
pub struct MessagePreview {
    pub text: String,
    pub created_at: String, // ISO String
    pub is_read: bool,
}

#[derive(Serialize)]
pub struct MessageHistoryResponse {
    pub messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub text: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Interaction {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub action: String,
    pub context_type: Option<String>,
    pub context_id: Option<String>,
    pub comment: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// USERS PREFERENCES
#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeRange {
    pub min: i32,
    pub max: i32,
}

// #[derive(Deserialize, Clone, FromRow, Debug, Serialize)]
// pub struct GetPreferences {
//     pub age_range: Option<AgeRange>,
//     pub distance_max: Option<i32>,
//     pub gender_preference: Option<Vec<String>>,
//     pub ethnicity_preference: Option<Vec<String>>,
//     pub religion_preference: Option<Vec<String>>,
// }
