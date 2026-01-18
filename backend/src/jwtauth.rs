use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use actix_web::dev::ServiceRequest;
use actix_web::{Error as ActixError, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;

// Removed: FirebaseAuth is not being used currently
// use firebase_auth::{FirebaseAuth, FirebaseUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl Claims {
    pub fn create_new_token(claim: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::default(), claim, &EncodingKey::from_secret("secret".as_ref()))
    }

    pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
        Ok(token_data.claims)
    }

    pub async fn jwt_validator(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
        // get the token from request
        let token = credentials.token();

        // TODO: Add Firebase token verification here when ready
        // For now, just verify our custom JWT

        // verify the token
        match Self::verify_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                Ok(req)
            }
            Err(_) => Err((ErrorUnauthorized("Invalid token"), req)),
        }
    }
}