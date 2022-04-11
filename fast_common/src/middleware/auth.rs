#![allow(clippy::type_complexity)]

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

use actix_http::header::HeaderValue;
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse, Result,
};
use actix_web::dev::{forward_ready, Service, Transform};
use actix_web::web::head;
use futures::{FutureExt, StreamExt};
use futures::future::{LocalBoxFuture, ok, Ready};

use crate::common::api_result::Api;
use crate::config::toml_config::{Config, CONFIG};
use crate::utils::crypt_util::Claims;

async fn is_white_list(path: &str) -> bool {
    for x in CONFIG.whitelist.list.iter() {
        if x.eq(path) {
            println!("时代");
            return true;
        }
    }
    return false;
}


#[derive(Clone)]
pub struct Authorization;


impl<S, B> Transform<S, ServiceRequest> for Authorization where
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

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        B: MessageBody,
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
                    None => {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized().json(Api::from(actix_web::error::ErrorUnauthorized("未认证1"))).map_into_right_body()
                        ))
                    }
                    Some(header) => {
                        let result = Claims::validation_token(header.to_str().unwrap());
                        if result.is_err() {
                            Ok(req.into_response(
                                HttpResponse::Unauthorized().json(Api::from(actix_web::error::ErrorUnauthorized("未认证2"))).map_into_right_body()
                            ))
                        } else {
                            srv.call(req).await.map(|res| res.map_into_left_body())
                        }
                    }
                }
            } else {
                srv.call(req).await.map(|res| res.map_into_left_body())
            }
        }.boxed_local()
    }
}