#![allow(unused)]

use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;
use crate::models::inputs::UpdateProfileRequest;
use crate::db::{user_queries, profile_queries, images_queries, prompt_queries};

/// Sample profile data for seeding (based on data/profiles.ts)
struct SeedProfile {
    phone: String,
    email: String,
    name: String,
    pronouns: String,
    gender: String,
    sexuality: String,
    height: i32,
    job: String,
    ethnicity: String,
    politics: String,
    religion: String,
    relationship_type: String,
    dating_intention: String,
    drinks: String,
    smokes: String,
    // Preferences for matching
    preferences: SeedPreferences,
    // Prompts (question, answer)
    prompts: Vec<(String, String)>,
    // Image URLs (from Unsplash for demo)
    images: Vec<String>,
}

/// Preferences for matching algorithm
struct SeedPreferences {
    age_min: i32,
    age_max: i32,
    distance_max: i32,
    gender_preference: Vec<String>,
    ethnicity_preference: Vec<String>,
    religion_preference: Vec<String>,
}

/// Get sample profiles data with preferences, prompts, and images (based on data/profiles.ts)
fn get_seed_profiles() -> Vec<SeedProfile> {
    vec![
        // === WOMEN (from profiles.ts) ===
        SeedProfile {
            phone: "+1111111111".to_string(),
            email: "ana@example.com".to_string(),
            name: "Ana".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 168, // 5'6"
            job: "Actress".to_string(),
            ethnicity: "Latina".to_string(),
            politics: "Moderate".to_string(),
            religion: "Catholic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Sometimes".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 25,
                age_max: 40,
                distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![],
                religion_preference: vec![],
            },
            prompts: vec![
                ("The way to win me over is".to_string(), "Good conversation and even better food".to_string()),
                ("Unusual skills".to_string(), "I can speak 3 languages fluently".to_string()),
                ("I'm looking for".to_string(), "Someone genuine who loves adventures".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=400".to_string(),
                "https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2222222222".to_string(),
            email: "scarlett@example.com".to_string(),
            name: "Scarlett".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 160, // 5'3"
            job: "Actress".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Agnostic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 28,
                age_max: 45,
                distance_max: 100,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![],
                religion_preference: vec![],
            },
            prompts: vec![
                ("The way to win me over is".to_string(), "Making me laugh uncontrollably".to_string()),
                ("My simple pleasures".to_string(), "Coffee, good books, and rainy days".to_string()),
                ("Dating me is like".to_string(), "A rollercoaster you never want to get off".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=400".to_string(),
                "https://images.unsplash.com/photo-1531746020798-e6953c6e8e04?w=400".to_string(),
                "https://images.unsplash.com/photo-1502823403499-6ccfcf4fb453?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+3333333333".to_string(),
            email: "elizabeth@example.com".to_string(),
            name: "Elizabeth".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 163, // 5'4"
            job: "Actress".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Spiritual".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Sometimes".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 30,
                age_max: 50,
                distance_max: 75,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec!["White".to_string()],
                religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "I can cook a 5-course meal in an hour".to_string()),
                ("The way to win me over is".to_string(), "Being passionate about something".to_string()),
                ("I'm looking for".to_string(), "A best friend I'm also attracted to".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+5555555555".to_string(),
            email: "sadie@example.com".to_string(),
            name: "Sadie".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Bisexual".to_string(),
            height: 152, // 5'0"
            job: "Actress".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Agnostic".to_string(),
            relationship_type: "Figuring out my dating goals".to_string(),
            dating_intention: "Figuring out my dating goals".to_string(),
            drinks: "Sometimes".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 20,
                age_max: 35,
                distance_max: 50,
                gender_preference: vec!["Man".to_string(), "Woman".to_string()],
                ethnicity_preference: vec![],
                religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "surviving on iced coffee".to_string()),
                ("I'm looking for".to_string(), "Someone who gets my weird humor".to_string()),
                ("The way to win me over is".to_string(), "Taking me to a concert".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1531746020798-e6953c6e8e04?w=400".to_string(),
                "https://images.unsplash.com/photo-1502823403499-6ccfcf4fb453?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+6666666666".to_string(),
            email: "disha@example.com".to_string(),
            name: "Disha".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 170, // 5'7"
            job: "Actress".to_string(),
            ethnicity: "South Asian".to_string(),
            politics: "Moderate".to_string(),
            religion: "Hindu".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "No".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 25,
                age_max: 40,
                distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec!["South Asian".to_string()],
                religion_preference: vec!["Hindu".to_string()],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Dancing in the rain".to_string()),
                ("The way to win me over is".to_string(), "Being fit and having a good sense of humor".to_string()),
                ("I'm convinced that".to_string(), "Good vibes attract good people".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
            ],
        },
        // === MEN (from profiles.ts) ===
        SeedProfile {
            phone: "+7777777777".to_string(),
            email: "chrise@example.com".to_string(),
            name: "Chris E".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 183, // 6'0"
            job: "Actor".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 25,
                age_max: 40,
                distance_max: 100,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![],
                religion_preference: vec![],
            },
            prompts: vec![
                ("The way to win me over is".to_string(), "Love my dog as much as I do".to_string()),
                ("I'm looking for".to_string(), "Someone who appreciates a good pun".to_string()),
                ("My simple pleasures".to_string(), "Morning runs with my dog".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+8888888888".to_string(),
            email: "chrish@example.com".to_string(),
            name: "Chris H".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 190, // 6'3"
            job: "Actor".to_string(),
            ethnicity: "White".to_string(),
            politics: "Moderate".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Sometimes".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 25,
                age_max: 45,
                distance_max: 150,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![],
                religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "I can surf any wave".to_string()),
                ("The way to win me over is".to_string(), "Loving the outdoors as much as me".to_string()),
                ("I'm convinced that".to_string(), "Australia has the best beaches".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1480455624313-e29b44bbfde1?w=400".to_string(),
                "https://images.unsplash.com/photo-1504257432389-52343af06ae3?w=400".to_string(),
                "https://images.unsplash.com/photo-1463453091185-61582044d556?w=400".to_string(),
                "https://images.unsplash.com/photo-1489980557514-251d61e3eeb6?w=400".to_string(),
                "https://images.unsplash.com/photo-1492447166138-50c3889fccb1?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+9999999999".to_string(),
            email: "henry@example.com".to_string(),
            name: "Henry".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 185, // 6'1"
            job: "Actor".to_string(),
            ethnicity: "White".to_string(),
            politics: "Moderate".to_string(),
            religion: "Catholic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Sometimes".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 22,
                age_max: 38,
                distance_max: 100,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![],
                religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "Building custom gaming PCs".to_string()),
                ("The way to win me over is".to_string(), "Being my player 2".to_string()),
                ("I'm looking for".to_string(), "Someone who appreciates fantasy and sci-fi".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1010101010".to_string(),
            email: "hrithik@example.com".to_string(),
            name: "Hrithik".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 180, // 5'11"
            job: "Actor".to_string(),
            ethnicity: "South Asian".to_string(),
            politics: "Moderate".to_string(),
            religion: "Hindu".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "No".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 22,
                age_max: 40,
                distance_max: 75,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec!["South Asian".to_string()],
                religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Dancing like nobody's watching".to_string()),
                ("The way to win me over is".to_string(), "Having rhythm and soul".to_string()),
                ("I'm convinced that".to_string(), "Dance is the ultimate expression".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1463453091185-61582044d556?w=400".to_string(),
                "https://images.unsplash.com/photo-1489980557514-251d61e3eeb6?w=400".to_string(),
                "https://images.unsplash.com/photo-1492447166138-50c3889fccb1?w=400".to_string(),
                "https://images.unsplash.com/photo-1504257432389-52343af06ae3?w=400".to_string(),
                "https://images.unsplash.com/photo-1480455624313-e29b44bbfde1?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1212121212".to_string(),
            email: "alex@example.com".to_string(),
            name: "Alex".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 178, // 5'10"
            job: "Creator".to_string(),
            ethnicity: "Latino".to_string(),
            politics: "Moderate".to_string(),
            religion: "Catholic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            preferences: SeedPreferences {
                age_min: 21,
                age_max: 35,
                distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec!["Latina".to_string()],
                religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "A fresh haircut and good coffee".to_string()),
                ("The way to win me over is".to_string(), "Having style and substance".to_string()),
                ("I'm looking for".to_string(), "A partner in life and business".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
            ],
        },
    ]
}

/// Convert SeedProfile to UpdateProfileRequest
fn to_profile_request(seed: &SeedProfile) -> UpdateProfileRequest {
    UpdateProfileRequest {
        name: Some(seed.name.clone()),
        bio: None,
        birthdate: None,
        pronouns: Some(seed.pronouns.clone()),
        gender: Some(seed.gender.clone()),
        sexuality: Some(seed.sexuality.clone()),
        height: Some(seed.height),
        location: None,
        job: Some(seed.job.clone()),
        company: None,
        school: None,
        ethnicity: Some(seed.ethnicity.clone()),
        politics: Some(seed.politics.clone()),
        religion: Some(seed.religion.clone()),
        relationship_type: Some(seed.relationship_type.clone()),
        dating_intention: Some(seed.dating_intention.clone()),
        drinks: Some(seed.drinks.clone()),
        smokes: Some(seed.smokes.clone()),
    }
}

/// Convert SeedPreferences to JSON
fn to_preferences_json(prefs: &SeedPreferences) -> serde_json::Value {
    json!({
        "ageRange": {
            "min": prefs.age_min,
            "max": prefs.age_max
        },
        "distanceMax": prefs.distance_max,
        "genderPreference": prefs.gender_preference,
        "ethnicityPreference": prefs.ethnicity_preference,
        "religionPreference": prefs.religion_preference
    })
}

/// Update user preferences in the database
async fn update_user_preferences(pool: &PgPool, user_id: &Uuid, preferences: serde_json::Value) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET preferences = $2 WHERE id = $1"
    )
    .bind(user_id)
    .bind(preferences)
    .execute(pool)
    .await?;

    Ok(())
}

/// Seed the database with sample data
/// This function will:
/// - Create new users OR get existing users by phone
/// - Create/update profiles
/// - Add images (6 per user)
/// - Add prompts (3 per user)
/// - Update preferences for all users
pub async fn seed_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    let profiles = get_seed_profiles();
    let count = profiles.len();
    let mut created_count = 0;
    let mut updated_count = 0;
    
    for seed in &profiles {
        // Get or create user
        let (user_id, is_new) = user_queries::get_or_create_user(pool, &seed.phone, &seed.email).await?;
        let user_uuid = Uuid::parse_str(&user_id).expect("Invalid UUID from get_or_create_user");
        
        // Create/update profile using existing function
        let profile_req = to_profile_request(seed);
        profile_queries::upsert_profile(pool, &user_uuid, &profile_req).await?;
        
        // Check if user already has images
        let existing_images = images_queries::count_images(pool, &user_uuid).await?;
        if existing_images == 0 {
            // Add 6 images using existing function
            for image_url in &seed.images {
                images_queries::upload_profile_images(pool, &user_uuid, image_url).await?;
            }
            println!("  📸 Added {} images", seed.images.len());
        }
        
        // Check if user already has prompts
        let existing_prompts = prompt_queries::count_prompts(pool, &user_uuid).await?;
        if existing_prompts == 0 {
            // Add 3 prompts using existing function
            for (question, answer) in &seed.prompts {
                prompt_queries::insert_prompt(pool, &user_uuid, question, answer).await?;
            }
            println!("  💬 Added {} prompts", seed.prompts.len());
        }
        
        // Update preferences
        let preferences_json = to_preferences_json(&seed.preferences);
        update_user_preferences(pool, &user_uuid, preferences_json).await?;
        
        if is_new {
            created_count += 1;
            println!("✨ Created: {} ({}) - Gender: {}, Looking for: {:?}", 
                seed.name, 
                user_id,
                seed.gender,
                seed.preferences.gender_preference
            );
        } else {
            updated_count += 1;
            println!("🔄 Updated: {} ({}) - Looking for: {:?}", 
                seed.name, 
                user_id,
                seed.preferences.gender_preference
            );
        }
    }
    
    println!("\n✅ Database seeding complete!");
    println!("  - {} new users created", created_count);
    println!("  - {} existing users updated", updated_count);
    println!("  - Total: {} profiles with full data", count);
    println!("\nUsers seeded:");
    println!("  - 5 Women (Ana, Scarlett, Elizabeth, Sadie, Disha)");
    println!("  - 5 Men (Chris E, Chris H, Henry, Hrithik, Alex)");
    println!("\nEach user has:");
    println!("  - Complete profile details");
    println!("  - 6 images");
    println!("  - 3 prompts");
    println!("  - Dating preferences (for feed filtering)");
    
    Ok(())
}
