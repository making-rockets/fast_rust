use std::cell::RefCell;
use std::future::{ready, Ready};
use std::rc::Rc;
use actix_web::Error;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::LocalBoxFuture;

pub struct HandleMethod;

pub struct HandleMethodMiddleAware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for HandleMethod
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
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

impl<S, B> Service<ServiceRequest> for HandleMethodMiddleAware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            println!("获取请求{:?}", &req);
            service.call(req).await
        })
    }
}
