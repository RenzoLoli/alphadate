use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{ErrorResponse, TagUpdate, UserUpdate},
    services::{TagService, UserService},
};

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all")]
async fn get_all_tags() -> impl Responder {
    HttpResponse::Ok().json(TagService::get_all())
}

#[get("")]
async fn get_tag_by_id(id_query: Result<web::Query<IdQuery>, actix_web::Error>) -> impl Responder {
    let id = match id_query {
        Ok(query) => query.id.clone(),
        Err(err) => return HttpResponse::BadRequest().json(ErrorResponse::new(err.to_string())),
    };

    match TagService::find_by_id(&id) {
        Some(tag) => HttpResponse::Ok().json(tag),
        None => HttpResponse::NotFound().json(ErrorResponse::new("Cannot get tag".to_owned())),
    }
}

#[put("/{id}")]
async fn update_tag(path: web::Path<String>, tag_update: web::Json<TagUpdate>) -> impl Responder {
    let id = path.into_inner();
    match TagService::update_tag(&id, &tag_update) {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(err.to_string())),
    }
}

#[delete("/{id}")]
async fn delete_tag(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match TagService::delete_tag(&id) {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse::new(err.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tags)
        .service(get_tag_by_id)
        .service(update_tag)
        .service(delete_tag);
}
