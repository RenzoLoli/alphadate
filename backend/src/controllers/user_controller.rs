use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{ErrorResponse, UserUpdate},
    services::UserService,
};

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all")]
async fn get_all_users() -> impl Responder {
    HttpResponse::Ok().json(UserService::get_all())
}

#[get("")]
async fn get_user_by_id(id_query: Result<web::Query<IdQuery>, actix_web::Error>) -> impl Responder {
    let id = match id_query {
        Ok(query) => query.id.clone(),
        Err(err) => return HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string())),
    };

    match UserService::find_by_id(&id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json(ErrorResponse::new("Cannot get user".to_owned())),
    }
}

#[put("/{id}")]
async fn update_user(
    path: web::Path<String>,
    user_update: web::Json<UserUpdate>,
) -> impl Responder {
    let id = path.into_inner();
    match UserService::update_user(&id, &user_update) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(err.to_string())),
    }
}

#[delete("/{id}")]
async fn delete_user(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match UserService::delete_user(&id) {
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
