use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::{error, Error};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{Future, ok, Ready};
use crate::utils::crypt_util::Claims;

pub struct Auth;

impl<S, B> Transform<S> for Auth
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
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

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::pin(async move {
            let token = req.headers().get("access_token");
            match token {
                None => {
                    match req.path() {
                        "/admin/index/login" | "/admin/index/send_reg_code" => {
                            svc.call(req).await
                        }
                        _ => { Err(error::ErrorUnauthorized(" required a auth header")) }
                    }
                }
                Some(access_token) => {
                    match access_token.to_str() {
                        Ok(access_token) => {
                            let result = Claims::validation_token(&access_token.to_string());
                            match result {
                                Ok(_) => { svc.call(req).await }
                                Err(e) => { Err(error::ErrorUnauthorized("认证过期或其他问题")) }
                            }
                        }
                        Err(e) => { Err(error::ErrorUnauthorized(e.to_string())) }
                    }
                }
            }
        })
    }
}
