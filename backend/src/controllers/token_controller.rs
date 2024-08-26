use actix_web::http::header::AUTHORIZATION;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use crate::controllers::resources::{ErrorResource, TokenResource};
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

    let token = match result {
        Some(token) => token,
        None => return HttpResponse::NotModified().json(ErrorResource::new("Cannot renew Token")),
    };

    let resource = TokenResource { token };

    HttpResponse::Ok().json(resource)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(renew);
}
