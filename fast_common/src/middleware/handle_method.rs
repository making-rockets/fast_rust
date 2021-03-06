use actix_web::dev::{Transform, Service, ServiceRequest, ServiceResponse};
use std::future::Future;
use actix_web::{error, Error, HttpMessage};
use actix_web::body::MessageBody;
use std::cell::{ RefCell};
use std::rc::Rc;
use futures::future::{Ready, ok};
use std::task::{Context, Poll};
use std::pin::Pin;
use actix_http::http::Method;

pub struct HandleMethod;
pub struct HandleMethodMiddleAware<S> {
    service: Rc<RefCell<S>>
}

impl<S, B> Transform<S> for HandleMethod where S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
                                               S::Future: 'static, B: MessageBody + 'static {
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = HandleMethodMiddleAware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(HandleMethodMiddleAware { service: Rc::new(RefCell::new(service)) })
    }
}

impl<S, B> Service for HandleMethodMiddleAware<S> where S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
                                                        S::Future: 'static,
                                                        B: MessageBody + 'static, {
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let mut svc = self.service.clone();
        Box::pin(async move {
            println!("获取请求{:?}", &req);
            let method = req.method();

            match method {
                &Method::GET => {
                    svc.call(req).await
                }
                &Method::POST => {
                    let result = req.mime_type();
                    match result {
                        Ok(option) => {
                            match option {
                                None => { svc.call(req).await }
                                Some(_) => { svc.call(req).await }
                            }
                        }

                        Err(_) => {
                            Err(error::ErrorUnsupportedMediaType("没有请求类型哦"))
                        }
                    }
                }
                &_ => {
                    svc.call(req).await
                }
            }
        })
    }
}
