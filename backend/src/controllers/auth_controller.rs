use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    domain::{ErrorResponse, OkResponse, TokenResponse, UserLogin, UserRegister},
    services::{AuthService, ContextServices},
};

#[post("/login")]
async fn login(
    services: ContextServices,
    user_req: Result<web::Json<UserLogin>, actix_web::Error>,
) -> impl Responder {
    let auth_service = &services.auth_service;
    let user_login = match user_req {
        Ok(user) => user.into_inner(),
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string()));
        }
    };

    let token = auth_service.login(user_login).await;

    match token {
        Ok(token) => HttpResponse::Ok().json(TokenResponse::new(token)),
        Err(err) => HttpResponse::NotFound().json(ErrorResponse::new(err)),
    }
}

#[post("/register")]
async fn register(
    services: ContextServices,
    user_req: Result<web::Json<UserRegister>, actix_web::Error>,
) -> impl Responder {
    let auth_service = &services.auth_service;
    log::error!("error");

    let user_register = match user_req {
        Ok(user) => user.into_inner(),
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string()));
        }
    };

    let user = auth_service.register(&user_register).await;

    match user {
        Ok(_) => HttpResponse::Ok().json(OkResponse::new("Registered Succesfully".to_owned())),
        Err(err) => HttpResponse::Conflict().json(ErrorResponse::new(err.to_string())),
    }
}

// #[post("/validate")]
// async fn validate(token_req: Result<web::Json<Token>, actix_web::Error>) -> impl Responder {
//     let token_res = match token_req {
//         Ok(token) => token.into_inner(),
//         Err(_) => {
//             return HttpResponse::BadRequest().json(ErrorResponse::new("Needed token".to_owned()));
//         }
//     };
//
//     let validate = TokenService::validate_token(token_res.token);
//
//     match validate {
//         Ok(_) => HttpResponse::Ok().json(OkResponse::new("Valid Token".to_owned())),
//         Err(_) => HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid Token".to_owned())),
//     }
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    //TODO: Add renew expired tokens service
    cfg.service(login).service(register);
}
