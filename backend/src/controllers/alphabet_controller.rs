use std::time::Duration;

use actix_web::{
    delete, get, patch, post, put,
    rt::{spawn, time::interval},
    web, HttpResponse, Responder,
};

use crate::{
    controllers::resources::{
        AlphabetCompleteResource, AlphabetCreateResource, AlphabetResource, AlphabetUpdateResource,
        ErrorResource, UserDateIsCompleteResource,
    },
    domain::{
        AlphabetAddDateIdeaCommand, AlphabetCreateCommand, AlphabetDeleteCommand,
        AlphabetRemoveDateIdeaCommand, AlphabetToggleCompleteDateCommand, AlphabetUpdateCommand,
        GetAllAlphabetsQuery, GetAlphabetByIdQuery, GetAlphabetByUserIdQuery,
    },
    services::{ContextServices, ServiceHandlerTrait},
};

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

    let resources = alphabets
        .into_iter()
        .map(AlphabetCompleteResource::from)
        .collect::<Vec<AlphabetCompleteResource>>();

    HttpResponse::Ok().json(resources)
}

#[get("/all/{user_id}/base")]
async fn get_all_alphabets_base(
    path: web::Path<(String,)>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_query_service = &services.alphabet_query_service;

    let (id,) = path.into_inner();
    log::debug!("Getting all alphabets base form for user <{}>", id.clone());
    let query = GetAlphabetByUserIdQuery { id: id.clone() };

    let alphabets = match alphabet_query_service.handle(query).await {
        Ok(date_ideas) => date_ideas,
        Err(err) => {
            return HttpResponse::NotFound().json(ErrorResource::new(err.to_string().as_str()));
        }
    };

    let resources = alphabets
        .into_iter()
        .map(AlphabetResource::from)
        .collect::<Vec<AlphabetResource>>();

    HttpResponse::Ok().json(resources)
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

    let resource = AlphabetCompleteResource::from(alphabet);

    HttpResponse::Ok().json(resource)
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

#[post("/{id}/date-idea/{date_idea_id}")]
async fn add_date_idea_to_alphabet(
    path: web::Path<(String, String)>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;
    let (alphabet_id, date_idea_id) = path.into_inner();

    let command = AlphabetAddDateIdeaCommand::from((alphabet_id.clone(), date_idea_id.clone()));

    log::debug!(
        "Adding date idea to alphabet <{}> <- <{}>",
        alphabet_id.clone(),
        date_idea_id.clone()
    );

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

#[delete("/{id}/date-idea/{date_idea_id}")]
async fn remove_date_idea_from_alphabet(
    path: web::Path<(String, String)>,
    services: ContextServices,
) -> impl Responder {
    let alphabet_command_service = &services.alphabet_command_service;
    let (alphabet_id, date_idea_id) = path.into_inner();

    let command = AlphabetRemoveDateIdeaCommand::from((alphabet_id.clone(), date_idea_id.clone()));

    log::debug!(
        "Removing date idea from alphabet <{}> <- <{}>",
        alphabet_id.clone(),
        date_idea_id.clone()
    );

    let alphabet = match alphabet_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = AlphabetResource::from(alphabet);

    HttpResponse::Ok().json(resource)
}

#[patch("/{id}/complete/{user_date_id}")]
async fn toggle_complete_date_from_alphabet(
    path: web::Path<(String, String)>,
    services: ContextServices,
) -> impl Responder {
    let user_date_command_service = &services.user_date_command_service;
    let (alphabet_id, user_date_id) = path.into_inner();

    let command = AlphabetToggleCompleteDateCommand {
        id: alphabet_id.clone(),
        user_date_id: user_date_id.clone(),
    };

    log::debug!(
        "Toggle complete date from alphabet <{}> <- <{}>",
        alphabet_id.clone(),
        user_date_id.clone()
    );

    let user_date = match user_date_command_service.handle(command).await {
        Ok(alphabet) => alphabet,
        Err(err) => {
            return HttpResponse::InternalServerError().json(ErrorResource::new(err.as_str()))
        }
    };

    let resource = UserDateIsCompleteResource::from(user_date);

    HttpResponse::Ok().json(resource)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_alphabets);
    cfg.service(get_all_alphabets_base);
    cfg.service(get_alphabet_by_id);
    cfg.service(create_alphabet);
    cfg.service(update_alphabet);
    cfg.service(delete_alphabet);
    cfg.service(add_date_idea_to_alphabet);
    cfg.service(remove_date_idea_from_alphabet);
    cfg.service(toggle_complete_date_from_alphabet);
}
