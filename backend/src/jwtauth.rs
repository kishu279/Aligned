use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl Claims {
    pub fn create_new_token(claim: Claims) -> Result<String, Error> {
        encode(&Header::default(), &claim, &EncodingKey::from_secret("secret".as_ref()))
    }

    pub fn verify_token(token: &str) -> Result<Claims, Error> {
        let token_data = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
        Ok(token_data.claims)
    }
}