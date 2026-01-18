use actix_web::{HttpRequest, error::ErrorUnauthorized};
use firebase_auth::{FirebaseAuth, FirebaseUser};

pub fn verify_request(
    req: &HttpRequest,
    firebase: &FirebaseAuth,
) -> Result<FirebaseUser, actix_web::Error> {
    // let firebase_auth = req.app_data::<FirebaseAuth>().expect("must init FirebaseAuth in Application Data. see description in https://crates.io/crates/firebase-auth");

    let header = req
        .headers()
        .get("Authorization")
        .ok_or(ErrorUnauthorized("Missing Authorization header"))?;

    let auth_str = header
        .to_str()
        .map_err(|_| ErrorUnauthorized("Invalid header"))?;

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(ErrorUnauthorized("Invalid auth format"))?;

    // println!("Verifying token: {}", token);

    // firebase.verify is NOT async - it's synchronous
    let user = match firebase.verify(token) {
        Ok(data) => data,
        Err(e) => return Err(ErrorUnauthorized(format!("Invalid token: {:?}", e))),
    };
    
    Ok(user)

    // match firebase_auth.verify(&token) {
    //     Err(e) => Err(ErrorUnauthorized(format!("Failed to verify Token {}", e))),
    //     Ok(user) => Ok(user),
    // }
}
