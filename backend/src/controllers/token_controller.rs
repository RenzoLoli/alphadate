use actix_web::http::header::AUTHORIZATION;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use crate::domain::{ErrorResponse, TokenResponse};
use crate::services::TokenService;

#[post("/renew")]
async fn renew(req: HttpRequest) -> impl Responder {
    let result = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .filter(|auth_str| auth_str.starts_with("Bearer "))
        .map(|auth_str| &auth_str["Bearer ".len()..])
        .and_then(|token| TokenService::renew_token(token.to_string()).ok());

    match result {
        Some(token) => HttpResponse::Ok().json(TokenResponse::new(token)),
        None => {
            HttpResponse::NotModified().json(ErrorResponse::new("Cannot renew Token".to_owned()))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(renew);
}
