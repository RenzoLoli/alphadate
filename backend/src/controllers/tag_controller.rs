use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    controllers::resources::{ErrorResource, TagResource},
    domain::{GetAllTagsQuery, GetTagByIdQuery, TagDeleteCommand, TagUpdateCommand},
    services::{ContextServices, ServiceHandlerTrait},
};

use super::resources::TagUpdateResource;

#[derive(Deserialize, Serialize)]
struct IdQuery {
    id: String,
}

#[get("/all")]
async fn get_all_tags(services: ContextServices) -> impl Responder {
    let tag_service = &services.tag_query_service;

    let query = GetAllTagsQuery {};

    let tags = match tag_service.handle(query).await {
        Ok(tags) => tags,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resources: Vec<TagResource> = tags.into_iter().map(TagResource::from).collect();

    HttpResponse::Ok().json(resources)
}

#[get("/{id}")]
async fn get_tag_by_id(
    id_query: web::Path<(String,)>,
    services: ContextServices,
) -> impl Responder {
    let tag_service = &services.tag_query_service;

    let id = id_query.into_inner().0;
    let query = GetTagByIdQuery { id };

    let tag = match tag_service.handle(query).await {
        Ok(tag) => tag,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resource = TagResource::from(tag);

    HttpResponse::Ok().json(resource)
}

#[put("/{id}")]
async fn update_tag(
    path: web::Path<String>,
    tag_update_resource: web::Json<TagUpdateResource>,
    services: ContextServices,
) -> impl Responder {
    let id = path.into_inner();
    let tag_command_service = &services.tag_command_service;

    let command = TagUpdateCommand::from((id, tag_update_resource.into_inner()));

    let tag = match tag_command_service.handle(command).await {
        Ok(tag) => tag,
        Err(err) => {
            return HttpResponse::NotModified().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resource = TagResource::from(tag);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}")]
async fn delete_tag(path: web::Path<(String,)>, services: ContextServices) -> impl Responder {
    let tag_command_service = &services.tag_command_service;

    let (id,) = path.into_inner();

    let command = TagDeleteCommand { id };

    let tag = match tag_command_service.handle(command).await {
        Ok(tag) => tag,
        Err(err) => {
            return HttpResponse::NotModified().json(ErrorResource::new(err.to_string().as_str()))
        }
    };

    let resource = TagResource::from(tag);

    HttpResponse::Ok().json(resource)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tags)
        .service(get_tag_by_id)
        .service(update_tag)
        .service(delete_tag);
}
