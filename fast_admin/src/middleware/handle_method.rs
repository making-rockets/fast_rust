use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error};
use futures::future::LocalBoxFuture;
use futures_util::{StreamExt, TryStreamExt};
use std::cell::RefCell;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::sync::Arc;

pub struct HandleMethod;

pub struct HandleMethodMiddleAware<S> {
    service: Arc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for HandleMethod
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
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
            service: Arc::new(RefCell::new(service)),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for HandleMethodMiddleAware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            // let (http_request, payload) = req.parts_mut();
            // let mut bytes = web::BytesMut::new();
            // while let Some(item) = payload.next().await {
            //     bytes.extend_from_slice(&item?);
            // }

            service.call(req).await
        })
    }
}
