use actix_http::Method;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};

use actix_web::http::Error;
use actix_web::{error, HttpMessage};
use futures::future::{ok, Ready};
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct HandleMethod;

pub struct HandleMethodMiddleAware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B, ServiceRequest> Transform<S, ServiceRequest> for HandleMethod
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = HandleMethodMiddleAware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(HandleMethodMiddleAware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

impl<S, B> Service<ServiceRequest> for HandleMethodMiddleAware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("actix-web middleware ------header = {:?}", req.head());
        let fut = self.service.call(req);
        Box::pin(async move { Ok(fut.await?) })
    }
}
