use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{
    body::BoxBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error, HttpResponse,
};

use crate::controllers::resources::ErrorResource;

type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

#[derive(Default)]
pub struct BadRequest;
pub struct BadRequestMiddleware<S> {
    service: S,
}

impl<S> Transform<S, ServiceRequest> for BadRequest
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = BadRequestMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(BadRequestMiddleware { service }))
    }
}

impl<S> Service<ServiceRequest> for BadRequestMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            if res.status() == StatusCode::BAD_REQUEST {
                log::warn!("Bad Request - {:?}", res.response().error());
                return Ok(res.into_response(
                    HttpResponse::BadRequest().json(ErrorResource::new("Bad Request")),
                ));
            };

            Ok(res)
        })
    }
}
