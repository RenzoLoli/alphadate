use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::resources::{ErrorResource, UserUpdateResource},
    domain::{GetAllUsersQuery, GetUserByIdQuery, UserDeleteCommand, UserUpdateCommand},
    services::{ContextServices, ServiceHandlerTrait},
};

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all")]
async fn get_all_users(services: ContextServices) -> impl Responder {
    let user_service = services.user_query_service.clone();

    let query = GetAllUsersQuery {};

    let users = match user_service.handle(query).await {
        Ok(users) => users,
        Err(_) => return HttpResponse::NotFound().json(ErrorResource::new("Cannot get users")),
    };

    HttpResponse::Ok().json(users)
}

#[get("{id}")]
async fn get_user_by_id(services: ContextServices, path: web::Path<(String,)>) -> impl Responder {
    let user_service = services.user_query_service.clone();

    let query = GetUserByIdQuery {
        id: path.into_inner().0,
    };

    let user = match user_service.handle(query).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().json(ErrorResource::new("Cannot get user")),
    };

    HttpResponse::Ok().json(user)
}

#[put("/{id}")]
async fn update_user(
    services: ContextServices,
    path: web::Path<(String,)>,
    user_update_resource: web::Json<UserUpdateResource>,
) -> impl Responder {
    let user_command_service = &services.user_command_service;

    let (id,) = path.into_inner();
    let command = UserUpdateCommand::from((id, user_update_resource.into_inner()));

    let user = match user_command_service.handle(command).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(ErrorResource::new(err.to_string().as_str()));
        }
    };

    HttpResponse::Ok().json(user)
}

#[delete("/{id}")]
async fn delete_user(services: ContextServices, path: web::Path<(String,)>) -> impl Responder {
    let user_command_service = &services.user_command_service;

    let (id,) = path.into_inner();

    let command = UserDeleteCommand { id };

    let user = match user_command_service.handle(command).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    HttpResponse::Ok().json(user)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // TODO: add VisualUser because password is shown in the response
    cfg.service(get_all_users)
        .service(get_user_by_id)
        .service(update_user)
        .service(delete_user);
}
