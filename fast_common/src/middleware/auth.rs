use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use actix_http::body::MessageBody;
use actix_http::header::HeaderValue;
use actix_web::middleware::DefaultHeaders;
use actix_web::{Error};


use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::service;

use futures::future::{ok, Future, Ready, LocalBoxFuture, err};
use crate::common::api_result::Api;
use crate::config::toml_config;


fn is_white_list(path: &str) -> bool {
    let whitelist = toml_config::CONFIG.whitelist;
    for x in whitelist.list.iter() {
        if x.eq(path) {
            return true;
        };
    }
    return false;
}

pub async fn checked_token(token: &str, path: &str) -> anyhow::Result<()> {
    //check token alive
    Ok(())
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,

{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S> where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let request_path = req.path();

            if is_white_list(request_path) {
                service.call(req).await
            } else {
                let Authorization = req.headers().get("Authorization");
                match Authorization {
                    Some(access_toen) => {
                        service.call(req).await
                    }
                    None => {
                        let result = Api::from(actix_web::error::ErrorUnauthorized("未授权")).to_response_of_json().await;
                        Ok(req.into_response(req.into_response(result)))
                    }
                }
            }
        })
    }
}

