use std::sync::Arc;

use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{ErrorResponse, UserUpdate},
    services::ContextServices,
};

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all")]
async fn get_all_users(services: ContextServices) -> impl Responder {
    let user_service = &services.user_service;
    HttpResponse::Ok().json(user_service.get_all().await)
}

#[get("")]
async fn get_user_by_id(
    services: ContextServices,
    id_query: Result<web::Query<IdQuery>, actix_web::Error>,
) -> impl Responder {
    let user_service = &services.user_service;

    let id = match id_query {
        Ok(query) => query.id.clone(),
        Err(err) => return HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string())),
    };

    match user_service.find_by_id(&id).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json(ErrorResponse::new("Cannot get user".to_owned())),
    }
}

#[put("/{id}")]
async fn update_user(
    services: ContextServices,
    path: web::Path<String>,
    user_update: web::Json<UserUpdate>,
) -> impl Responder {
    let user_service = &services.user_service;
    let id = path.into_inner();
    match user_service.update_user(&id, &user_update).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(err.to_string())),
    }
}

#[delete("/{id}")]
async fn delete_user(services: ContextServices, path: web::Path<String>) -> impl Responder {
    let user_service = &services.user_service;
    let id = path.into_inner();
    match user_service.delete_user(&id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(err.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users)
        .service(get_user_by_id)
        .service(update_user)
        .service(delete_user);
}
