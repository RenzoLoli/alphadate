use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::resources::{
        AlphabetAddDateIdeaResource, AlphabetCreateResource, AlphabetRemoveDateIdeaResource,
        AlphabetResource, AlphabetUpdateResource, ErrorResource,
    },
    domain::{
        AlphabetAddDateIdeaCommand, AlphabetCreateCommand, AlphabetDeleteCommand,
        AlphabetRemoveDateIdeaCommand, AlphabetUpdateCommand, GetAllAlphabetsQuery,
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
    log::debug!("Getting all alphabets for user <{}>", id);
    let query = GetAllAlphabetsQuery { user_id: id };

    let alphabets = match alphabet_query_service.handle(query).await {
        Ok(date_ideas) => date_ideas,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()));
        }
    };

    log::debug!("Alphabets -> {:?}", alphabets);
    HttpResponse::Ok().json(alphabets)
}

#[get("/{id}")]
async fn get_alphabet_by_id(
    path: web::Path<(String,)>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_query_service = &services.alphabet_query_service;

    let id = path.into_inner().0;
    log::debug!("Getting alphabet by id <{}>", id);
    let query = GetAlphabetByIdQuery { id };

    let alphabet = match alphabet_query_service.handle(query).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()));
        }
    };

    HttpResponse::Ok().json(alphabet)
}

#[post("")]
async fn create_alphabet(
    alphabet_create_resource: web::Json<AlphabetCreateResource>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;

    log::debug!("Creating alphabet");

    let command = AlphabetCreateCommand::from(alphabet_create_resource.into_inner());

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()));
        }
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

    log::debug!("Updating alphabet <{}>", id);
    let resource = alphabet_update_resource.into_inner();

    let command = AlphabetUpdateCommand::from((id, resource));

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}")]
async fn delete_alphabet(path: web::Path<(String,)>, services: ContextServices) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;

    let (id,) = path.into_inner();

    let command = AlphabetDeleteCommand { id: id.clone() };

    log::debug!("Deleting alphabet <{}>", id);

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

#[post("/{id}")]
async fn add_date_idea_to_alphabet(
    path: web::Path<(String,)>,
    alphabet_add_date_idea_resource: web::Json<AlphabetAddDateIdeaResource>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;
    let id = path.into_inner().0;
    let resource = alphabet_add_date_idea_resource.into_inner();

    let command = AlphabetAddDateIdeaCommand::from((id.clone(), resource));

    log::debug!("Adding date idea to alphabet <{}>", id.clone());

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}")]
async fn remove_date_idea_from_alphabet(
    path: web::Path<(String,)>,
    alphabet_remove_date_idea_resource: web::Json<AlphabetRemoveDateIdeaResource>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;
    let id = path.into_inner().0;
    let resource = alphabet_remove_date_idea_resource.into_inner();

    let command = AlphabetRemoveDateIdeaCommand::from((id.clone(), resource));

    log::debug!("Removing date idea from alphabet <{}>", id.clone());

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_alphabets);
    cfg.service(get_alphabet_by_id);
    cfg.service(create_alphabet);
    cfg.service(update_alphabet);
    cfg.service(delete_alphabet);
    cfg.service(add_date_idea_to_alphabet);
    cfg.service(remove_date_idea_from_alphabet);
}
