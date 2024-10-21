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

    log::debug!("Logging in user <{}>", singin_command.email);

    let token = match auth_command_service.handle(singin_command).await {
        Ok(token) => token,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resource = TokenResource::new(token.as_str());

    HttpResponse::Ok().json(resource)
}

#[post("/register")]
async fn register(
    services: ContextServices,
    user_req: web::Json<SignUpResource>,
) -> impl Responder {
    let auth_service = &services.auth_command_service;

    let user_command = SignUpCommand::from(user_req.into_inner());

    log::debug!("Registering user <{}>", user_command.email);

    let _user = match auth_service.handle(user_command).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resource = OkResource::new("Registered Succesfully");

    HttpResponse::Ok().json(resource)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(register);
}
