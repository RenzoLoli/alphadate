use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::resources::{
        DateIdeaAddTagResource, DateIdeaCompleteResource, DateIdeaResource, DateIdeaUpdateResource,
        ErrorResource,
    },
    domain::{
        DateIdeaAddTagCommand, DateIdeaCreateCommand, DateIdeaDeleteCommand,
        DateIdeaRemoveTagCommand, DateIdeaUpdateCommand, GetAllDateIdeasQuery,
        GetDateIdeaByIdQuery, GetRandomDateIdeaQuery,
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

    log::debug!("Getting all date ideas");

    let query = GetAllDateIdeasQuery {};

    let ideas = match date_idea_query_service.handle(query).await {
        Ok(date_ideas) => date_ideas,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resources = ideas
        .into_iter()
        .map(DateIdeaCompleteResource::from)
        .collect::<Vec<DateIdeaCompleteResource>>();

    HttpResponse::Ok().json(resources)
}

#[derive(Deserialize)]
struct RandomIdeaQuery {
    #[serde(default)]
    exclude_active: bool,
}

#[get("/random/{alphabet_id}")]
async fn get_random_date_idea(
    path: web::Path<(String,)>,
    params: web::Query<RandomIdeaQuery>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_query_service = &services.date_idea_query_service;

    let (alphabet_id,) = path.into_inner();

    let exclude_active = params.into_inner().exclude_active;

    log::debug!("Getting random date idea");

    let query = GetRandomDateIdeaQuery {
        alphabet_id,
        exclude_active,
    };

    let date_idea = match date_idea_query_service.handle(query).await {
        Ok(date_idea) => date_idea,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resource = DateIdeaCompleteResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[get("/{id}")]
async fn get_date_idea_by_id(
    path: web::Path<(String,)>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_query_service = &services.date_idea_query_service;
    let id = path.into_inner().0;

    log::debug!("Getting date idea with id: {}", id);

    let query = GetDateIdeaByIdQuery { id };

    let date_idea = match date_idea_query_service.handle(query).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    let resource = DateIdeaCompleteResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[post("")]
async fn create_date_idea(
    date_idea_create_resource: web::Json<DateIdeaCreateResource>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;

    let command = DateIdeaCreateCommand::from(date_idea_create_resource.into_inner());

    log::debug!("Creating date idea");

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
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

    log::debug!("Updating date idea with id: {}", id.clone());

    let command = DateIdeaUpdateCommand::from((id, resource));

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = DateIdeaResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}")]
async fn delete_date_idea(path: web::Path<(String,)>, services: ContextServices) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;

    let (id,) = path.into_inner();

    log::debug!("Deleting date idea with id: {}", id.clone());

    let command = DateIdeaDeleteCommand { id };

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = DateIdeaResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[post("/{id}/tag")]
async fn add_tag_to_date_idea(
    path: web::Path<(String,)>,
    date_idea_add_tag_resource: web::Json<DateIdeaAddTagResource>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;
    let id = path.into_inner().0;
    let resource = (id.clone(), date_idea_add_tag_resource.into_inner());

    let command = DateIdeaAddTagCommand::from(resource);

    log::debug!("Adding tag to date idea with id: {}", id.clone());

    let date_idea = match date_idea_command_service.handle(command).await {
        Ok(date_idea) => date_idea,
        Err(err) => return HttpResponse::NotFound().json(ErrorResource::new(err.as_str())),
    };

    let resource = DateIdeaResource::from(date_idea);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}/tag")]
async fn remove_tag_from_date_idea(
    path: web::Path<(String,)>,
    date_idea_remove_tag_resource: web::Json<DateIdeaRemoveTagResource>,
    services: ContextServices,
) -> impl Responder {
    let date_idea_command_service = &services.date_idea_command_service;
    let id = path.into_inner().0;
    let resource = (id.clone(), date_idea_remove_tag_resource.into_inner());

    let command = DateIdeaRemoveTagCommand::from(resource);

    log::debug!("Removing tag from date idea with id: {}", id.clone());

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
        .service(delete_date_idea)
        .service(get_random_date_idea);
}
