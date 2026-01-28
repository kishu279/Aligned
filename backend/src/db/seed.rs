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
    // New fields
    bio: String,
    birthdate: String,
    location: String,
    company: String,
    school: String,
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
            bio: "Aspiring actress loving life in LA. 🌟".to_string(),
            birthdate: "1997-04-30".to_string(),
            location: "34.0522,-118.2437".to_string(), // Los Angeles
            company: "Freelance".to_string(),
            school: "School of Dramatic Arts".to_string(),
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
            bio: "New York soul living in an LA world. 🎭".to_string(),
            birthdate: "1984-11-22".to_string(),
            location: "34.0522,-118.2437".to_string(), // LA (moved)
            company: "Studio productions".to_string(),
            school: "NYU Tisch".to_string(),
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
            bio: "Classic cinema enthusiast. 🎬".to_string(),
            birthdate: "1989-02-16".to_string(),
            location: "34.0522,-118.2437".to_string(),
            company: "Marvel Studios".to_string(),
            school: "LAMDA".to_string(),
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
            bio: "Stranger Things have happened... 🚲".to_string(),
            birthdate: "2002-04-16".to_string(),
            location: "34.0522,-118.2437".to_string(),
            company: "Netflix".to_string(),
            school: "Homeschooled".to_string(),
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
            bio: "Fitness enthusiast and dancer. 💃".to_string(),
            birthdate: "1992-06-13".to_string(),
            location: "19.0760,72.8777".to_string(), // Mumbai
            company: "Bollywood".to_string(),
            school: "Whistling Woods".to_string(),
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
            bio: "Captain of my own ship. 🛡️".to_string(),
            birthdate: "1981-06-13".to_string(),
            location: "42.3601,-71.0589".to_string(), // Boston
            company: "Disney".to_string(),
            school: "Sudbury Regional".to_string(),
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
            bio: "Thunder god on weekdays, surfer on weekends. ⚡".to_string(),
            birthdate: "1983-08-11".to_string(),
            location: "-33.8688,151.2093".to_string(), // Sydney
            company: "Marvel".to_string(),
            school: "Heathmont College".to_string(),
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
            bio: "Gamer, builder of PCs, lover of fantasy. ⚔️".to_string(),
            birthdate: "1983-05-05".to_string(),
            location: "51.5074,-0.1278".to_string(), // London
            company: "Warhammer".to_string(),
            school: "Stowe School".to_string(),
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
            bio: "Greek God of Dance. 🕺".to_string(),
            birthdate: "1974-01-10".to_string(),
            location: "19.0760,72.8777".to_string(), // Mumbai
            company: "YRF".to_string(),
            school: "Sydenham College".to_string(),
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
            bio: "Creating content and living the dream. 🎥".to_string(),
            birthdate: "1998-03-24".to_string(),
            location: "34.0522,-118.2437".to_string(),
            company: "YouTube".to_string(),
            school: "UCLA".to_string(),
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
        // === ADDITIONAL WOMEN ===
        SeedProfile {
            phone: "+1313131313".to_string(),
            email: "sophia@example.com".to_string(),
            name: "Sophia".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 165,
            job: "Product Designer".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Agnostic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Designing for the future. 🎨".to_string(),
            birthdate: "1996-08-15".to_string(),
            location: "37.7749,-122.4194".to_string(), // San Francisco
            company: "Figma".to_string(),
            school: "RISD".to_string(),
            preferences: SeedPreferences {
                age_min: 24, age_max: 35, distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("The way to win me over is".to_string(), "Critique my kerning".to_string()),
                ("My simple pleasures".to_string(), "Matcha lattes and Moleskine notebooks".to_string()),
                ("I'm looking for".to_string(), "Someone who appreciates good design".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
                "https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1414141414".to_string(),
            email: "olivia@example.com".to_string(),
            name: "Olivia".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Bisexual".to_string(),
            height: 172,
            job: "Chef".to_string(),
            ethnicity: "Black".to_string(),
            politics: "Left".to_string(),
            religion: "Spiritual".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Frequently".to_string(),
            smokes: "Sometimes".to_string(),
            bio: "Cooking up a storm. 🍳".to_string(),
            birthdate: "1994-03-12".to_string(),
            location: "40.7128,-74.0060".to_string(), // NYC
            company: "Le Coucou".to_string(),
            school: "CIA".to_string(),
            preferences: SeedPreferences {
                age_min: 25, age_max: 40, distance_max: 25,
                gender_preference: vec!["Man".to_string(), "Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "I can name every spice in a dish blindfolded".to_string()),
                ("The way to win me over is".to_string(), "Doing the dishes".to_string()),
                ("My simple pleasures".to_string(), "Late night tacos".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
                "https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1515151515".to_string(),
            email: "mia@example.com".to_string(),
            name: "Mia".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 158,
            job: "Nurse".to_string(),
            ethnicity: "Asian".to_string(),
            politics: "Moderate".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Marriage".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Saving lives and taking names. 🏥".to_string(),
            birthdate: "1999-12-05".to_string(),
            location: "34.0522,-118.2437".to_string(), // LA
            company: "Cedars-Sinai".to_string(),
            school: "UCLA Nursing".to_string(),
            preferences: SeedPreferences {
                age_min: 24, age_max: 32, distance_max: 30,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("I'm looking for".to_string(), "Someone patient and kind".to_string()),
                ("My simple pleasures".to_string(), "Sleeping in after a night shift".to_string()),
                ("The way to win me over is".to_string(), "Foot massages".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1554151228-14d9def656ec?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1502823403499-6ccfcf4fb453?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1616161616".to_string(),
            email: "isabella@example.com".to_string(),
            name: "Isabella".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 168,
            job: "Lawyer".to_string(),
            ethnicity: "Latina".to_string(),
            politics: "Conservative".to_string(),
            religion: "Catholic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Justice never sleeps. ⚖️".to_string(),
            birthdate: "1993-05-20".to_string(),
            location: "25.7617,-80.1918".to_string(), // Miami
            company: "Big Law Firm".to_string(),
            school: "Yale Law".to_string(),
            preferences: SeedPreferences {
                age_min: 30, age_max: 45, distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec!["Catholic".to_string(), "Christian".to_string()],
            },
            prompts: vec![
                ("I'm convinced that".to_string(), "I will win every argument".to_string()),
                ("The way to win me over is".to_string(), "Intelligence and ambition".to_string()),
                ("My simple pleasures".to_string(), "Red wine and true crime documentaries".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1717171717".to_string(),
            email: "ava@example.com".to_string(),
            name: "Ava".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 170,
            job: "Artist".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Agnostic".to_string(),
            relationship_type: "Polyamory".to_string(),
            dating_intention: "Casual".to_string(),
            drinks: "Often".to_string(),
            smokes: "Yes".to_string(),
            bio: "Creating chaos on canvas. 🎨".to_string(),
            birthdate: "2000-01-01".to_string(),
            location: "52.5200,13.4050".to_string(), // Berlin
            company: "Self-employed".to_string(),
            school: "UdK Berlin".to_string(),
            preferences: SeedPreferences {
                age_min: 20, age_max: 35, distance_max: 20,
                gender_preference: vec!["Man".to_string(), "Woman".to_string(), "Non-binary".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "I can sleep for 14 hours straight".to_string()),
                ("The way to win me over is".to_string(), "Ticket to an underground rave".to_string()),
                ("I'm looking for".to_string(), "Muse".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1517365830460-955ce3ccd263?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1818181818".to_string(),
            email: "charlotte@example.com".to_string(),
            name: "Charlotte".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 162,
            job: "Teacher".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "No".to_string(),
            smokes: "No".to_string(),
            bio: "Shaping young minds. 🍎".to_string(),
            birthdate: "1995-09-09".to_string(),
            location: "51.5074,-0.1278".to_string(), // London
            company: "Primary School".to_string(),
            school: "Oxford".to_string(),
            preferences: SeedPreferences {
                age_min: 25, age_max: 35, distance_max: 20,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Tea and biscuits".to_string()),
                ("I'm looking for".to_string(), "Someone wholesome".to_string()),
                ("The way to win me over is".to_string(), "Being good with kids".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1529626455594-4ff0802cfb7e?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+1919191919".to_string(),
            email: "amelia@example.com".to_string(),
            name: "Amelia".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Lesbian".to_string(),
            height: 175,
            job: "Pilot".to_string(),
            ethnicity: "White".to_string(),
            politics: "Moderate".to_string(),
            religion: "Atheist".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Catch me if you can. ✈️".to_string(),
            birthdate: "1988-07-20".to_string(),
            location: "33.9416,-118.4085".to_string(), // LAX
            company: "Delta".to_string(),
            school: "Embry-Riddle".to_string(),
            preferences: SeedPreferences {
                age_min: 28, age_max: 42, distance_max: 500,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "I can land a plane in a storm".to_string()),
                ("The way to win me over is".to_string(), "Packing your bags in 5 minutes".to_string()),
                ("My simple pleasures".to_string(), "Duty-free shopping".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1523950704592-3e44b01030e7?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2020202020".to_string(),
            email: "harper@example.com".to_string(),
            name: "Harper".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 160,
            job: "Writer".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Agnostic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Often".to_string(),
            smokes: "Sometimes".to_string(),
            bio: "Weaving words and drinking wine. 🍷".to_string(),
            birthdate: "1991-10-31".to_string(),
            location: "45.5051,-122.6750".to_string(), // Portland
            company: "Freelance".to_string(),
            school: "Reed College".to_string(),
            preferences: SeedPreferences {
                age_min: 28, age_max: 45, distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Typewriter sounds".to_string()),
                ("I'm looking for".to_string(), "Someone mysterious".to_string()),
                ("I'm convinced that".to_string(), "I was born in the wrong decade".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1481214110833-d4e30f1528b6?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2121212121".to_string(),
            email: "evelyn@example.com".to_string(),
            name: "Evelyn".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 167,
            job: "Architect".to_string(),
            ethnicity: "Asian".to_string(),
            politics: "Moderate".to_string(),
            religion: "Buddhist".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Marriage".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Building dreams. 🏛️".to_string(),
            birthdate: "1987-02-14".to_string(),
            location: "41.8781,-87.6298".to_string(), // Chicago
            company: "SOM".to_string(),
            school: "Cornell".to_string(),
            preferences: SeedPreferences {
                age_min: 35, age_max: 50, distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Symmetry".to_string()),
                ("I'm looking for".to_string(), "A solid foundation".to_string()),
                ("The way to win me over is".to_string(), "Appreciating historic buildings".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1534751516642-a1af1ef26a56?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2222212121".to_string(),
            email: "abigail@example.com".to_string(),
            name: "Abigail".to_string(),
            pronouns: "she/her".to_string(),
            gender: "Woman".to_string(),
            sexuality: "Straight".to_string(),
            height: 164,
            job: "Veterinarian".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Animal lover. 🐾".to_string(),
            birthdate: "1994-06-25".to_string(),
            location: "30.2672,-97.7431".to_string(), // Austin
            company: "Austin Vet Care".to_string(),
            school: "TAMU".to_string(),
            preferences: SeedPreferences {
                age_min: 25, age_max: 38, distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("The way to win me over is".to_string(), "Being kind to animals".to_string()),
                ("My simple pleasures".to_string(), "Puppy breath".to_string()),
                ("I'm looking for".to_string(), "Someone to walk dogs with".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1479936343636-73cdc5aae0c3?w=400".to_string(),
                "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400".to_string(),
                "https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400".to_string(),
                "https://images.unsplash.com/photo-1488426862026-3ee34a7d66df?w=400".to_string(),
                "https://images.unsplash.com/photo-1517841905240-472988babdf9?w=400".to_string(),
                "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?w=400".to_string(),
            ],
        },
        // === ADDITIONAL MEN ===
        SeedProfile {
            phone: "+2323232323".to_string(),
            email: "liam@example.com".to_string(),
            name: "Liam".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 182,
            job: "Firefighter".to_string(),
            ethnicity: "White".to_string(),
            politics: "Conservative".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Marriage".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Running into danger while you run out. 🚒".to_string(),
            birthdate: "1994-05-10".to_string(),
            location: "40.7128,-74.0060".to_string(), // NYC
            company: "FDNY".to_string(),
            school: "Academy".to_string(),
            preferences: SeedPreferences {
                age_min: 22, age_max: 30, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("The way to win me over is".to_string(), "A home cooked meal".to_string()),
                ("I'm looking for".to_string(), "Someone loyal".to_string()),
                ("Unusual skills".to_string(), "I can carry you down a ladder".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2424242424".to_string(),
            email: "noah@example.com".to_string(),
            name: "Noah".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 175,
            job: "Software Engineer".to_string(),
            ethnicity: "Asian".to_string(),
            politics: "Liberal".to_string(),
            religion: "Agnostic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Debugging life one line at a time. 💻".to_string(),
            birthdate: "1997-12-12".to_string(),
            location: "37.7749,-122.4194".to_string(), // SF
            company: "Google".to_string(),
            school: "Stanford".to_string(),
            preferences: SeedPreferences {
                age_min: 24, age_max: 34, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Clean code and fast wifi".to_string()),
                ("I'm looking for".to_string(), "Player 2".to_string()),
                ("I'm convinced that".to_string(), "Tabs are better than spaces".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1463453091185-61582044d556?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2525252525".to_string(),
            email: "oliver@example.com".to_string(),
            name: "Oliver".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Gay".to_string(),
            height: 180,
            job: "Interior Designer".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Atheist".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Often".to_string(),
            smokes: "No".to_string(),
            bio: "Making spaces beautiful. 🏠".to_string(),
            birthdate: "1990-09-30".to_string(),
            location: "34.0522,-118.2437".to_string(), // LA
            company: "Self-employed".to_string(),
            school: "Parsons".to_string(),
            preferences: SeedPreferences {
                age_min: 25, age_max: 45, distance_max: 50,
                gender_preference: vec!["Man".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Mid-century modern furniture".to_string()),
                ("The way to win me over is".to_string(), "A good martini".to_string()),
                ("I'm looking for".to_string(), "Someone with taste".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1496345875659-11f7dd282d1d?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2626262626".to_string(),
            email: "elijah@example.com".to_string(),
            name: "Elijah".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 188,
            job: "Musician".to_string(),
            ethnicity: "Black".to_string(),
            politics: "Moderate".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Casual".to_string(),
            dating_intention: "Short-term fun".to_string(),
            drinks: "Socially".to_string(),
            smokes: "Yes".to_string(),
            bio: "Here for a good time, not a long time. 🎸".to_string(),
            birthdate: "2001-01-20".to_string(),
            location: "36.1627,-86.7816".to_string(), // Nashville
            company: "Indie Label".to_string(),
            school: "Berklee".to_string(),
            preferences: SeedPreferences {
                age_min: 21, age_max: 30, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Live music and cold beer".to_string()),
                ("The way to win me over is".to_string(), "Come to my show".to_string()),
                ("Unusual skills".to_string(), "I can play 5 instruments".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1506803682981-6e718a9dd3ee?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2727272727".to_string(),
            email: "william@example.com".to_string(),
            name: "William".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 185,
            job: "Doctor".to_string(),
            ethnicity: "White".to_string(),
            politics: "Conservative".to_string(),
            religion: "Catholic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Marriage".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Dedicated to healing. 👨‍⚕️".to_string(),
            birthdate: "1988-11-15".to_string(),
            location: "42.3601,-71.0589".to_string(), // Boston
            company: "Mass General".to_string(),
            school: "Harvard Med".to_string(),
            preferences: SeedPreferences {
                age_min: 28, age_max: 38, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("I'm looking for".to_string(), "Someone family oriented".to_string()),
                ("My simple pleasures".to_string(), "Golf on Sundays".to_string()),
                ("The way to win me over is".to_string(), "Intelligence".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1480429370139-e0132c086e2a?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2828282828".to_string(),
            email: "james@example.com".to_string(),
            name: "James".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 178,
            job: "Entrepreneur".to_string(),
            ethnicity: "Asian".to_string(),
            politics: "Moderate".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Building the next big thing. 🚀".to_string(),
            birthdate: "1995-04-04".to_string(),
            location: "37.7749,-122.4194".to_string(), // SF
            company: "Tech Startup".to_string(),
            school: "YCS 24".to_string(),
            preferences: SeedPreferences {
                age_min: 24, age_max: 34, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "I can pitch anything".to_string()),
                ("The way to win me over is".to_string(), "Being ambitious".to_string()),
                ("My simple pleasures".to_string(), "IPO day".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1515202913167-d9538f8d6f09?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+2929292929".to_string(),
            email: "benjamin@example.com".to_string(),
            name: "Benjamin".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 183,
            job: "Accountant".to_string(),
            ethnicity: "White".to_string(),
            politics: "Conservative".to_string(),
            religion: "Jewish".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Marriage".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Balanced books and a balanced life. 📊".to_string(),
            birthdate: "1992-02-28".to_string(),
            location: "40.7128,-74.0060".to_string(), // NYC
            company: "Deloitte".to_string(),
            school: "Wharton".to_string(),
            preferences: SeedPreferences {
                age_min: 26, age_max: 36, distance_max: 20,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec!["White".to_string()],
                religion_preference: vec!["Jewish".to_string()],
            },
            prompts: vec![
                ("I'm convinced that".to_string(), "Excel is a lifestyle".to_string()),
                ("The way to win me over is".to_string(), "Being organized".to_string()),
                ("My simple pleasures".to_string(), "Tax returns".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1480455624313-e29b44bbfde1?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+3030303030".to_string(),
            email: "lucas@example.com".to_string(),
            name: "Lucas".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 193,
            job: "Basketball Player".to_string(),
            ethnicity: "Black".to_string(),
            politics: "Liberal".to_string(),
            religion: "Christian".to_string(),
            relationship_type: "Casual".to_string(),
            dating_intention: "Short-term fun".to_string(),
            drinks: "No".to_string(),
            smokes: "No".to_string(),
            bio: "Ball is life. 🏀".to_string(),
            birthdate: "2002-11-22".to_string(),
            location: "34.0522,-118.2437".to_string(), // LA
            company: "Lakers G-League".to_string(),
            school: "Duke".to_string(),
            preferences: SeedPreferences {
                age_min: 20, age_max: 30, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("Unusual skills".to_string(), "I can dunk".to_string()),
                ("The way to win me over is".to_string(), "Coming to my games".to_string()),
                ("My simple pleasures".to_string(), "Winning".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1504257432389-52343af06ae3?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+3131313131".to_string(),
            email: "henry2@example.com".to_string(),
            name: "Henry".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 170,
            job: "Writer".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Agnostic".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Often".to_string(),
            smokes: "No".to_string(),
            bio: "Searching for the perfect sentence. ✍️".to_string(),
            birthdate: "1985-07-07".to_string(),
            location: "51.5074,-0.1278".to_string(), // London
            company: "Publisher".to_string(),
            school: "Cambridge".to_string(),
            preferences: SeedPreferences {
                age_min: 30, age_max: 45, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("My simple pleasures".to_string(), "Old bookstores".to_string()),
                ("The way to win me over is".to_string(), "Intelligence and wit".to_string()),
                ("I'm looking for".to_string(), "My equal".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1499996860823-5214fcc65f8f?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
        SeedProfile {
            phone: "+3232323232".to_string(),
            email: "theodore@example.com".to_string(),
            name: "Theodore".to_string(),
            pronouns: "he/him".to_string(),
            gender: "Man".to_string(),
            sexuality: "Straight".to_string(),
            height: 175,
            job: "Professor".to_string(),
            ethnicity: "White".to_string(),
            politics: "Liberal".to_string(),
            religion: "Atheist".to_string(),
            relationship_type: "Monogamy".to_string(),
            dating_intention: "Long-term relationship".to_string(),
            drinks: "Socially".to_string(),
            smokes: "No".to_string(),
            bio: "Teaching the next generation. 📚".to_string(),
            birthdate: "1980-03-15".to_string(),
            location: "42.3601,-71.0589".to_string(), // Boston
            company: "MIT".to_string(),
            school: "Princeton".to_string(),
            preferences: SeedPreferences {
                age_min: 35, age_max: 50, distance_max: 50,
                gender_preference: vec!["Woman".to_string()],
                ethnicity_preference: vec![], religion_preference: vec![],
            },
            prompts: vec![
                ("The way to win me over is".to_string(), "Curiosity".to_string()),
                ("I'm looking for".to_string(), "A partner in crime".to_string()),
                ("My simple pleasures".to_string(), "Solving equations".to_string()),
            ],
            images: vec![
                "https://images.unsplash.com/photo-1487309078313-fad80c3ec1e5?w=400".to_string(),
                "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400".to_string(),
                "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400".to_string(),
                "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400".to_string(),
                "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400".to_string(),
                "https://images.unsplash.com/photo-1492562080023-ab3db95bfbce?w=400".to_string(),
            ],
        },
    ]
}

/// Convert SeedProfile to UpdateProfileRequest
fn to_profile_request(seed: &SeedProfile) -> UpdateProfileRequest {
    UpdateProfileRequest {
        name: Some(seed.name.clone()),
        bio: Some(seed.bio.clone()),
        birthdate: Some(seed.birthdate.clone()),
        pronouns: Some(seed.pronouns.clone()),
        gender: Some(seed.gender.clone()),
        sexuality: Some(seed.sexuality.clone()),
        height: Some(seed.height),
        location: Some(seed.location.clone()),
        job: Some(seed.job.clone()),
        company: Some(seed.company.clone()),
        school: Some(seed.school.clone()),
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
        // Get or create user (using seed-specific function with fake Firebase UID)
        let (user_id, is_new) = user_queries::get_or_create_user_for_seed(pool, &seed.phone, &seed.email).await?;
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
