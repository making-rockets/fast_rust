use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};


use actix_web::{ Error};


use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};

use futures::future::{ok, Future, Ready};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
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

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("actix-web middleware ------header = {:?}", &req.head());

        let x = req.path();

        let    svc = self.service.call(req);
        Box::pin(async move {

            Ok(svc.await?)
            
            // let token = req.headers().get("Authorization");
            // match token {
            //     None => match req.path() {
            //         "/admin/index/login" | "/admin/index/send_reg_code" => {
            //             return svc.call(req).await;
            //         }
            //         _ => {
            //             let error = error::ErrorUnauthorized("required a Authorization token");
            //             let mut api = Api::from(error);
            //             return Ok(req.into_response(api.to_response_of_json().await));
            //         }
            //     },
            //     Some(access_token) => match access_token.to_str() {
            //         Ok(access_token) => {
            //             let result = Claims::validation_token(&access_token.to_string());
            //             match result {
            //                 Ok(_) => svc.call(req).await,
            //                 Err(e) => {
            //                     let error = error::ErrorUnauthorized(e.to_string());
            //                     let mut api = Api::from(error);
            //                     Ok(req.into_response(api.to_response_of_json().await))
            //                 }
            //             }
            //         }
            //         Err(e) => {
            //             let error = error::ErrorInternalServerError(e.to_string());
            //             let mut api = Api::from(error);
            //             Ok(req.into_response(api.to_response_of_json().await))
            //         }
            //     },
            // }
        })
    }
}
