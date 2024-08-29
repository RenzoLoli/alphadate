use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::resources::{
        DateIdeaAddTagResource, DateIdeaResource, DateIdeaUpdateResource, ErrorResource,
    },
    domain::{
        DateIdeaAddTagCommand, DateIdeaCreateCommand, DateIdeaDeleteCommand,
        DateIdeaRemoveTagCommand, DateIdeaUpdateCommand, GetAllDateIdeasQuery,
        GetDateIdeaByIdQuery,
    },
    services::{ContextServices, ServiceHandlerTrait},
};

use super::resources::{DateIdeaCreateResource, DateIdeaRemoveTagResource};

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all")]
async fn get_all_date_ideas(services: ContextServices) -> impl Responder {
    let date_idea_query_service = &services.date_idea_query_service;

    let query = GetAllDateIdeasQuery {};

    let ideas = match date_idea_query_service.handle(query).await {
        Ok(date_ideas) => date_ideas,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    HttpResponse::Ok().json(ideas)
}

#[get("/{id}")]
async fn get_date_idea_by_id(
    path: web::Path<(String,)>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_query_service = &services.date_idea_query_service;
    let id = path.into_inner().0;

    let query = GetDateIdeaByIdQuery { id };

    let date_idea = match date_idea_query_service.handle(query).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    HttpResponse::Ok().json(date_idea)
}

#[post("")]
async fn create_date_idea(
    date_idea_create_resource: web::Json<DateIdeaCreateResource>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;

    let command = DateIdeaCreateCommand::from(date_idea_create_resource.into_inner());

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    let resource = DateIdeaResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[put("/{id}")]
async fn update_date_idea(
    path: web::Path<(String,)>,
    date_idea_update_resource: web::Json<DateIdeaUpdateResource>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;
    let id = path.into_inner().0;
    let resource = date_idea_update_resource.into_inner();

    let command = DateIdeaUpdateCommand::from((id, resource));

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotModified().json(ErrorResource::new(err.as_str())),
    };

    HttpResponse::Ok().json(date_idea)
}

#[delete("/{id}")]
async fn delete_date_idea(path: web::Path<(String,)>, services: ContextServices) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;

    let (id,) = path.into_inner();

    let command = DateIdeaDeleteCommand { id };

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotModified().json(ErrorResource::new(err.as_str())),
    };

    let resource = DateIdeaResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[post("/{id}")]
async fn add_tag_to_date_idea(
    path: web::Path<(String,)>,
    date_idea_add_tag_resource: web::Json<DateIdeaAddTagResource>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;
    let id = path.into_inner().0;
    let resource = (id, date_idea_add_tag_resource.into_inner());

    let command = DateIdeaAddTagCommand::from(resource);

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    let resource = DateIdeaResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}")]
async fn remove_tag_from_date_idea(
    path: web::Path<(String,)>,
    date_idea_remove_tag_resource: web::Json<DateIdeaRemoveTagResource>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;
    let id = path.into_inner().0;
    let resource = (id, date_idea_remove_tag_resource.into_inner());

    let command = DateIdeaRemoveTagCommand::from(resource);

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    let resource = DateIdeaResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_date_ideas)
        .service(get_date_idea_by_id)
        .service(create_date_idea)
        .service(update_date_idea)
        .service(add_tag_to_date_idea)
        .service(remove_tag_from_date_idea)
        .service(delete_date_idea);
}
