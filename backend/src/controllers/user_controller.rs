use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{domain::ErrorResponse, services::UserService};

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all")]
async fn users() -> impl Responder {
    HttpResponse::Ok().json(UserService::get_all())
}

#[get("")]
async fn user(id_query: Result<web::Query<IdQuery>, actix_web::Error>) -> impl Responder {
    let id = match id_query {
        Ok(query) => query.id.clone(),
        Err(err) => return HttpResponse::BadRequest().json(err.to_string()),
    };

    match UserService::find_by_id(&id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json(ErrorResponse::new("User Not found".to_owned())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(users).service(user);
}
