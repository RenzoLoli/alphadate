use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    controllers::resources::{
        ErrorResource, OkResource, SignInResource, SignUpResource, TokenResource,
    },
    domain::{SignInCommand, SignUpCommand},
    services::{ContextServices, ServiceHandlerTrait},
};

#[post("/login")]
async fn login(
    services: ContextServices,
    signin_resource: web::Json<SignInResource>,
) -> impl Responder {
    let auth_command_service = &services.auth_command_service;

    let singin_command = SignInCommand::from(signin_resource.into_inner());

    let token = auth_command_service.handle(singin_command).await;

    match token {
        Ok(token) => HttpResponse::Ok().json(TokenResource::new(token.as_str())),
        Err(err) => HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str())),
    }
}

#[post("/register")]
async fn register(
    services: ContextServices,
    user_req: web::Json<SignUpResource>,
) -> impl Responder {
    let auth_service = &services.auth_command_service;

    let user_command = SignUpCommand::from(user_req.into_inner());

    let user = auth_service.handle(user_command).await;

    match user {
        Ok(_) => HttpResponse::Ok().json(OkResource::new("Registered Succesfully")),
        Err(err) => HttpResponse::Conflict().json(ErrorResource::new(err.to_string().as_str())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(register);
}
