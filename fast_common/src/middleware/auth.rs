use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::HeaderValue;
use actix_web::{error, Error, HttpMessage};
use futures::future::{ok, Future, Ready};

use crate::models::user::User;
use crate::utils::redis_util::RedisUtil;
use std::ops::Deref;

async fn get_user_from_redis(token: &String) -> Result<User, &str> {
    let result = RedisUtil::get_redis_util().await.get_json::<User>(token).await;
    return Ok(User {
        id: None,
        user_name: None,
        age: None,
        create_time: None,
    });
}

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
                        "/admin/index/login" => {
                            svc.call(req).await
                        }
                        _ => { Err(error::ErrorUnauthorized("please transmit a access_token header")) }
                    }
                }
                Some(access_token) => {
                    let result = access_token.to_str().unwrap();
                    //let user = get_user_from_redis(&result.to_string()).await.or_else(|_| Err(error::ErrorUnauthorized("找不到user")));
                    req.extensions_mut().insert("user");
                    svc.call(req).await
                    //Ok(svc.call(req).await?)
                }
            }
        })
    }
}
