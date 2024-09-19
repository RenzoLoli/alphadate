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

use crate::{controllers::resources::ErrorResource, services::TokenService};

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
        let token_result = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .filter(|auth_str| auth_str.starts_with("Bearer "))
            .map(|auth_str| &auth_str["Bearer ".len()..])
            .and_then(|token| TokenService::validate_token(token.to_string()).ok());

        if token_result.is_none() {
            return Box::pin(async move {
                let json =
                    HttpResponse::Unauthorized().json(ErrorResource::new("Needed Authentication"));
                Ok(req.into_response(json))
            });
        }

        let fut = self.service.call(req);
        Box::pin(fut)
    }
}
