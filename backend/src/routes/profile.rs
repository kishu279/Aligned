#![allow(unused)]

use actix_web::{HttpRequest, HttpResponse, HttpMessage, Responder, web};
use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use std::fs;
use std::path::Path;

use crate::models::outputs::{UserProfile, UserImage, UserPrompt};
use crate::models::inputs::{UpdateProfileRequest, UploadUrlRequest, DownloadRequest};
use crate::jwtauth::Claims;
use crate::models::outputs::{StatusResponse, FinalizeProfileResponse};
use crate::db::{profile_queries, prompt_queries, images_queries, user_queries};

use firebase_auth::FirebaseUser;
use crate::file_storage::{FileService, SignedUrlResponse, DownloadResponse};

pub async fn get_profile(
    pool: web::Data<PgPool>, 
    req: HttpRequest,
    file_service: web::Data<FileService>,
) -> impl Responder {
    let user: FirebaseUser = match req.extensions().get::<FirebaseUser>().cloned() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().json(StatusResponse {
            status: "error".to_string(),
            message: Some("Unauthorized".to_string()),
        })
    };
    
    println!("User requested {}", user.sub);
    println!("User email {:?}", user.email);

    // Get user_id from database using email
    let user_id = match user_queries::get_user_id_by_email(&pool, user.email.as_deref()).await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().json(StatusResponse {
            status: "error".to_string(),
            message: Some("User not found".to_string()),
        }),
        Err(e) => return HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Database error: {}", e)),
        })
    };

    // Get profile details (returns None if not found)
    let profile_details = match profile_queries::get_profile(&pool, &user_id).await {
        Ok(p) => Some(p),
        Err(_) => None,
    };

    // Get images and resolve download URLs
    let user_images = match images_queries::get_user_images(&pool, &user_id).await {
        Ok(rows) => {
            let mut images = Vec::new();
            for (id, key, order) in rows {
                // Get presigned download URL for each image
                let download_url = match file_service.download_file(&key).await {
                    Ok(response) => response.download_url,
                    Err(_) => key, // Fallback to key if download URL fails
                };
                images.push(UserImage {
                    id: id.to_string(),
                    url: download_url,
                    order,
                });
            }
            Some(images)

            // rows.into_iter().map(|(id, key, order)| UserImage {
            //     id: id.to_string(),
            //     url: key,
            //     order,
            // }).collect() // error
        },
        Err(_) => None,
    };

    println!("User images: {:?}", user_images);

    // Get prompts and map to UserPrompt structs
    let user_prompts = match prompt_queries::get_user_prompts(&pool, &user_id).await {
        Ok(rows) => Some(rows.into_iter().map(|(id, question, answer, order)| UserPrompt {
            id: id.to_string(),
            question,
            answer,
            order,
        }).collect()),
        Err(_) => None,
    };

    let user_profile = UserProfile {
        id: user_id.to_string(),
        images: user_images,
        prompts: user_prompts,
        details: profile_details,
    };

    HttpResponse::Ok().json(user_profile)
}

pub async fn update_profile(
    req: HttpRequest,
    body: web::Json<UpdateProfileRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    println!("Profile: Updating - Name: {:?}", body.name);

    // Get the user from Firebase auth
    let user: FirebaseUser = match req.extensions().get::<FirebaseUser>().cloned() {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Unauthorized".to_string()),
            });
        }
    };

    // Get user_id from database using email
    let user_id = match crate::db::user_queries::get_user_id_by_email(&pool, user.email.as_deref()).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            return HttpResponse::NotFound().json(StatusResponse {
                status: "error".to_string(),
                message: Some("User not found".to_string()),
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(format!("Database error: {}", e)),
            });
        }
    };

    // Create or update profile
    match profile_queries::upsert_profile(&pool, &user_id, &body).await {
        Ok(_) => {
            HttpResponse::Ok().json(StatusResponse {
                status: "success".to_string(),
                message: Some("Profile updated successfully".to_string()),
            })
        }
        Err(e) => {
            println!("Failed to upsert profile: {:?}", e);
            HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
            })
        }
    }
}

// WORKING
pub async fn get_upload_url(
    pool: web::Data<PgPool>, 
    req: HttpRequest, 
    body: web::Json<UploadUrlRequest>,
    file_service: web::Data<FileService>,
) -> impl Responder {
    println!("Upload URL: {}", body.filename);
    println!("Upload URL: {}", body.content_type);

    let Some(user) = req.extensions().get::<FirebaseUser>().cloned() else {
        return HttpResponse::Unauthorized().json(StatusResponse {
            status: "error".to_string(),
            message: Some("No authentication claims found".to_string()),
        })
    };

    let user_id = match user_queries::get_user_id_by_email(&pool, user.email.as_deref()).await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().json(StatusResponse {
            status: "error".to_string(),
            message: Some("User not found".to_string()),
        }),
        Err(e) => return HttpResponse::InternalServerError().json(StatusResponse {
            status: "error".to_string(),
            message: Some(format!("Database error: {}", e)),
        })
    };

    let (upload_url, key) = match file_service.upload_file_url(&body.filename, &body.content_type).await {
        Ok(res) => (res.upload_url, res.key),
        Err(e) => return HttpResponse::InternalServerError().json(SignedUrlResponse {
            upload_url: "".to_string(),
            key: "".to_string(),
        })
    };

    // save the url and key need to change the db
    images_queries::upload_profile_images(&pool, &user_id, &key).await;

    HttpResponse::Ok().json(SignedUrlResponse {
        upload_url,
        key,
    })
}

// pub async fn view_profile_images(
//     pool: web::Data<PgPool>, 
//     req: HttpRequest, 
//     body: web::Json<UploadUrlRequest>,
//     file_service: web::Data<FileService>,
// ) -> impl Responder {
//     let Some(user) = req.extensions().get::<FirebaseUser>().cloned() else {
//         return HttpResponse::Unauthorized().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("No authentication claims found".to_string()),
//         })
//     };

//     let user_id = match user_queries::get_user_id_by_email(&pool, user.email.as_deref()).await {
//         Ok(Some(id)) => id,
//         Ok(None) => return HttpResponse::NotFound().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("User not found".to_string()),
//         }),
//         Err(e) => return HttpResponse::InternalServerError().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some(format!("Database error: {}", e)),
//         })
//     };

//     // TODO: Implement view_profile_images logic
//     let user_images = match images_queries::get_user_images(&pool, &user_id).await {
//         Some(u) => 
//     }

//     match file_service.download_file(&body.key).await {
//         Ok(response) => HttpResponse::Ok().json(response),
//         Err(_) => HttpResponse::NotFound().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("File not found".to_string()),
//         })
//     }
// }

// WORKING
pub async fn get_download_url(
    req: HttpRequest, 
    body: web::Json<DownloadRequest>, 
    pool: web::Data<PgPool>,
    file_service: web::Data<FileService>,
) -> impl Responder {
    println!("Download URL: {}", body.key);

    match file_service.download_file(&body.key).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::NotFound().json(StatusResponse {
            status: "error".to_string(),
            message: Some("File not found".to_string()),
        })
    }
}

// REMOVED - Legacy function, use get_upload_url instead
// pub async fn upload_profile_images(pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<UploadUrlRequest>) -> impl Responder {
//     // Multipart handling would go here

//     let Some(claims) = req.extensions().get::<Claims>().cloned() else {
//         return HttpResponse::Unauthorized().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("No authentication claims found".to_string()),
//         })
//     };

//     let user_id: Uuid = match Uuid::parse_str(&claims.sub) {
//         Ok(id) => id,
//         Err(_) => {
//             return HttpResponse::BadRequest().json(StatusResponse {
//                 status: "error".to_string(),
//                 message: Some("Invalid user ID format".to_string()),
//             })
//         }
//     };

//     match images_queries::upload_profile_images(&pool, &user_id, &body.image_url).await {
//         Ok(_) => HttpResponse::Ok().json(StatusResponse {
//             status: "success".to_string(),
//             message: Some("Profile images uploaded successfully".to_string()),
//         }),
//         Err(e) => {
//             println!("Failed to upload profile images: {:?}", e);
//             HttpResponse::InternalServerError().json(StatusResponse {
//                 status: "error".to_string(),
//                 message: Some(e.to_string()),
//             })
//         }
//     }
// }

pub async fn finalize_profile(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {

    let Some(claims) = req.extensions().get::<Claims>().cloned() else {
        return HttpResponse::Unauthorized().json(FinalizeProfileResponse {
            status: "error".to_string(),
            message: Some("No authentication claims found".to_string()),
            pending_actions: Some(vec!["No authentication claims found".to_string()]),
        })
    };

    let user_id: Uuid = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(FinalizeProfileResponse {
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
                pending_actions: Some(vec!["Invalid user ID format".to_string()])
            })
        }
    };

    let mut pending: Vec<String> = Vec::new();

    // CHECK: all the 6 photos uploaded
    let images_uploaded = match images_queries::count_images(&pool, &user_id).await {
        Ok(count) => count,
        Err(e) => {
            println!("Failed to count images: {:?}", e);
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(e.to_string()),
            });
        }
    };

    if images_uploaded < 6 {
        pending.push(format!("Upload {} more images", 6 - images_uploaded));
    }

    // CHECK: all the 3 prompts uploaded
    let prompts_uploaded = match prompt_queries::count_prompts(&pool, &user_id).await {
        Ok(count) => count,
        Err(e) => {
            println!("Failed to count prompts: {:?}", e);
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(e.to_string()),
            });
        }
    };
    if prompts_uploaded < 3 {
        pending.push(format!("Upload {} more prompts", 3 - prompts_uploaded));
    }

    // CHECK: all profile details filled
    let missing_fields = match profile_queries::check_profile_attributes_filled(&pool, &user_id).await {
        Ok(count) => count,
        Err(e) => {
            println!("Failed to check profile attributes: {:?}", e);
            return HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some(e.to_string()),
            });
        }
    };
    if missing_fields > 0 {
        pending.push(format!("Fill {} more profile details", missing_fields));
    }

    if pending.is_empty() {
        HttpResponse::Ok().json(FinalizeProfileResponse {
            status: "success".to_string(),
            message: Some("Profile finalized successfully".to_string()),
            pending_actions: Some(pending),
        })
    } else {
        HttpResponse::BadRequest().json(FinalizeProfileResponse {
            status: "error".to_string(),
            message: Some("Profile not finalized".to_string()),
            pending_actions: Some(pending),
        })
    }
}

pub async fn delete_account(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> impl Responder {
    // Get user ID from claims
    let claims = match req.extensions().get::<Claims>().cloned() {
        Some(c) => c,
        None => {
            return HttpResponse::Unauthorized().json(StatusResponse {
                status: "error".to_string(),
                message: Some("No authentication claims found".to_string()),
            });
        }
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
            });
        }
    };

    // Delete user and profile and images and prompts
    match profile_queries::delete_user(&pool, &user_id).await {
        Ok(_) => {
            HttpResponse::Ok().json(StatusResponse {
                status: "success".to_string(),
                message: Some("Account deleted successfully".to_string()),
            })
        }
        Err(e) => {
            println!("Failed to delete account: {:?}", e);
            HttpResponse::InternalServerError().json(StatusResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
            })
        }
    }
}

#[derive(Deserialize, Debug)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
pub struct ImageUpload {
    #[multipart(limit = "50mb")]
    file: TempFile,
    metadata: MpJson<Metadata>,
}


// RAM IMAGE BINARYS
// pub async fn upload_user_images(pool: web::Data<PgPool>, req: HttpRequest, MultipartForm(form): MultipartForm<ImageUpload>) -> impl Responder {
//     let Some(claims) = req.extensions().get::<Claims>().cloned() else {
//         return HttpResponse::Unauthorized().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("No authentication claims found".to_string()),
//         });
//     };

//     let Ok(user_id) = Uuid::parse_str(&claims.sub) else {
//         return HttpResponse::BadRequest().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("Invalid user ID format".to_string()),
//         });
//     };

//     let file_name = form.file.file_name.as_ref().map(|s| s.as_str()).unwrap_or("uploaded_file");
//     let dest_path = Path::new("./uploads/").join(file_name);

//     println!("Path : {:?}", dest_path);

//     match fs::copy(&form.file.file.path(), &dest_path) {
//         Ok(_) => {
//             println!("File Uploaded!!!!");
//             HttpResponse::Ok().json(StatusResponse {
//                 status: "success".to_string(),
//                 message: Some("File uploaded successfully".to_string()),
//             })
//         },
//         Err(e) => {
//             println!("Failed to upload file: {:?}", e);
//             HttpResponse::InternalServerError().json(StatusResponse {
//                 status: "error".to_string(),
//                 message: Some("Failed to upload file".to_string()),
//             })
//         }
//     }
// }

// pub async fn upload_profile_images(pool: web::Data<PgPool>, req: HttpRequest, MultipartForm(form): MultipartForm<ImageUpload>) -> impl Responder {
//     let user: FirebaseUser = match req.extensions().get::<FirebaseUser>().cloned() {
//         Some(data) => data,
//         None => HttpResponse::InternalServerError().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("Invalid user ID in database".to_string()),
//         })
//     };

//     let Ok(user_id) = Uuid::parse_str(&user.uid) else {
//         return HttpResponse::BadRequest().json(StatusResponse {
//             status: "error".to_string(),
//             message: Some("Invalid user ID format".to_string()),
//         });
//     };

//     let file_name = form.file.file_name.as_ref().map(|s| s.as_ref()).unwrap_or("uploaded_file");
//     let content_type = form.content_type.as_ref().map(|s| s.as_ref()).unwrap_or("application/octet-stream");

//     match FileService::upload_file(&file_name, &form.file.file, content_type).await {
//         Ok(response) => {
//             HttpResponse::Ok().json(response)
//         },
//         Err(e) => {
//             HttpResponse::InternalServerError().json(StatusResponse {
//                 status: "error".to_string(),
//                 message: Some(e.to_string()),
//             })
//         }
//     }
// }