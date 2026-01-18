use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web::Data,
};
use firebase_auth::FirebaseAuth;
use futures::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;

use super::verifier::verify_request;

// Middleware factory
pub struct FirebaseAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for FirebaseAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = FirebaseAuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(FirebaseAuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct FirebaseAuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for FirebaseAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Get FirebaseAuth from app data
            let firebase = match req.app_data::<Data<FirebaseAuth>>() {
                Some(fb) => fb,
                None => {
                    let response =
                        HttpResponse::InternalServerError().body("FirebaseAuth not configured");
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            // Verify the token
            match verify_request(req.request(), firebase.get_ref()) {
                Ok(user) => {
                    // Store user in request extensions
                    req.extensions_mut().insert(user);

                    // Continue to the next service
                    let res = service.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                Err(e) => {
                    let response =
                        HttpResponse::Unauthorized().body(format!("Authentication failed: {}", e));
                    Ok(req.into_response(response).map_into_right_body())
                }
            }
        })
    }
}
