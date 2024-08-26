use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::resources::{
        AlphabetCreateResource, AlphabetResource, AlphabetUpdateResource, ErrorResource,
    },
    domain::{
        AlphabetCreateCommand, AlphabetDeleteCommand, AlphabetUpdateCommand, GetAllAlphabetsQuery,
        GetAlphabetByIdQuery,
    },
    services::{ContextServices, ServiceHandlerTrait},
};

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all/{user_id}")]
async fn get_all_alphabets(
    path: web::Path<(String,)>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_query_service = &services.alphabet_query_service;

    let (id,) = path.into_inner();
    let query = GetAllAlphabetsQuery { user_id: id };

    let alphabets = match alphabet_query_service.handle(query).await {
        Ok(date_ideas) => date_ideas,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    HttpResponse::Ok().json(alphabets)
}

#[get("/{id}")]
async fn get_alphabet_by_id(
    path: web::Path<(String,)>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_query_service = &services.alphabet_query_service;

    let id = path.into_inner().0;
    let query = GetAlphabetByIdQuery { id };

    let alphabet = match alphabet_query_service.handle(query).await {
        Ok(alphabet) => alphabet,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    HttpResponse::Ok().json(alphabet)
}

#[post("")]
async fn create_alphabet(
    alphabet_create_resource: web::Json<AlphabetCreateResource>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;

    let command = AlphabetCreateCommand::from(alphabet_create_resource.into_inner());

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

#[put("/{id}")]
async fn update_alphabet(
    path: web::Path<(String,)>,
    alphabet_update_resource: web::Json<AlphabetUpdateResource>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;
    let id = path.into_inner().0;
    let resource = alphabet_update_resource.into_inner();

    let command = AlphabetUpdateCommand::from((id, resource));

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => return HttpResponse::NotModified().json(ErrorResource::new(err.as_str())),
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}")]
async fn delete_alphabet(path: web::Path<(String,)>, services: ContextServices) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;

    let (id,) = path.into_inner();

    let command = AlphabetDeleteCommand { id };

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => return HttpResponse::NotModified().json(ErrorResource::new(err.as_str())),
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_alphabets)
        .service(get_alphabet_by_id)
        .service(update_alphabet)
        .service(delete_alphabet);
}
