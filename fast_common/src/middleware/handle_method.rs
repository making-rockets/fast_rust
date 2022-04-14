use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::future::{Ready, ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_http::{HttpMessage, Method};
use actix_http::body::MessageBody;
use actix_http::header::HeaderValue;
use actix_web::{Error, error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::middleware::DefaultHeaders;
use actix_web::web::service;
use futures::future::LocalBoxFuture;

use crate::common::api_result::Api;
use crate::config::toml_config;

pub struct HandleMethod;

pub struct HandleMethodMiddleAware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for HandleMethod where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = HandleMethodMiddleAware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HandleMethodMiddleAware {
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for HandleMethodMiddleAware<S> where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self,  req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            println!("获取请求{:?}", &req);
            service.call(req).await
        })
    }
}
