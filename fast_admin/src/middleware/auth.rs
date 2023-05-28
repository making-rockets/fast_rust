#![allow(clippy::type_complexity)]

use std::cell::RefCell;
use std::rc::Rc;

use actix_web::dev::{forward_ready, Service, Transform};
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse, Result,
};

use futures::future::{ok, LocalBoxFuture, Ready};
use futures::FutureExt;

use crate::common::api_result::Api;
use crate::config::toml_config::CONFIG;

async fn is_white_list(path: &str) -> bool {
    let mut is_access = false;
    for x in CONFIG.whitelist.list.iter() {
        if path.starts_with(&x.to_string()) {
            is_access = true;
        }
    }
    is_access
}

#[derive(Clone)]
pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthorizationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizationMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthorizationMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S> where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    B: MessageBody
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        async move {
            if !is_white_list(req.path()).await {
                let option = req.headers().get("Authorization");
                match option {
                    None => Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(Api::from(actix_web::error::ErrorUnauthorized("未认证")))
                            .map_into_right_body(),
                    )),
                    Some(header) => {
                        if false {
                            Ok(req.into_response(
                                HttpResponse::Unauthorized()
                                    .json(Api::from(actix_web::error::ErrorUnauthorized("未认证2")))
                                    .map_into_right_body(),
                            ))
                        } else {
                            srv.call(req).await.map(|res| res.map_into_left_body())
                        }
                    }
                }
            } else {
                srv.call(req).await.map(|res| res.map_into_left_body())
            }
        }
            .boxed_local()
    }
}
