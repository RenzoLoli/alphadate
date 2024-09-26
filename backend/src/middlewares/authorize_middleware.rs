use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{
    body::BoxBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    Error, HttpResponse,
};

use crate::{controllers::resources::ErrorResource, services::ContextServices};

type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

#[derive(Default)]
pub struct Authorize;
pub struct AuthorizeMiddleware<S> {
    service: S,
}

impl<S> Transform<S, ServiceRequest> for Authorize
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizeMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizeMiddleware { service }))
    }
}

impl<S> Service<ServiceRequest> for AuthorizeMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token_util_service = &req
            .app_data::<ContextServices>()
            .unwrap()
            .token_util_service;

        let token_req = match req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .filter(|auth_str| auth_str.starts_with("Bearer "))
            .inspect(|auth_str| log::debug!("Authorization -> {:#?}", auth_str))
            .map(|auth_str| &auth_str["Bearer ".len()..])
        {
            Some(token) => token,
            None => {
                return Box::pin(async move {
                    let json = HttpResponse::Unauthorized()
                        .json(ErrorResource::new("Needed Authentication"));
                    Ok(req.into_response(json))
                });
            }
        };

        if let Err(err) = token_util_service.validate_token(token_req.to_string()) {
            log::debug!("Error -> {:#?}", err);
            return Box::pin(async move {
                let json = HttpResponse::Unauthorized().json(ErrorResource::new(err.as_str()));
                Ok(req.into_response(json))
            });
        };

        let fut = self.service.call(req);
        Box::pin(fut)
    }
}
