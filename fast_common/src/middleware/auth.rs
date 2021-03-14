use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::{error, Error, HttpRequest, HttpResponse};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{Future, ok, Ready};
use crate::utils::crypt_util::Claims;
use crate::common::api_result::{Api, GlobalError};
use std::borrow::{Borrow, BorrowMut};
use actix_web::test::ok_service;
use actix_http::Response;

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
            let token = req.headers().get("Authorization");
            match token {
                None => {
                    match req.path() {
                        "/admin/index/login" | "/admin/index/send_reg_code" => {
                            return svc.call(req).await;
                        }
                        _ => {
                            let error1 = error::ErrorInternalServerError("required a Authorization token");
                            let response:Response = Api::<()>::from(Err(GlobalError("error1".to_string()))).await.to_response_of_json().await;

                            return Ok(req.into_response(response));
                            //Ok(req.into_response(response1)).into_service()
                        }
                    }
                }
                Some(access_token) => {
                    match access_token.to_str() {
                        Ok(access_token) => {
                            let result = Claims::validation_token(&access_token.to_string());
                            match result {
                                Ok(_) => { svc.call(req).await }
                                Err(e) => { Err(error::ErrorUnauthorized(e)) }
                            }
                        }
                        Err(e) => { Err(error::ErrorUnauthorized(e.to_string())) }
                    }
                }
            }
        })
    }
}
