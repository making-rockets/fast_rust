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

async fn get_user_from_redis<'a>(token: &'a String) -> Result<User, &str> {
    let redisUtil = RedisUtil::get_conn().await;
    let t = redisUtil.get_json::<User>(token).await;
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
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::pin(async move {
            let value = HeaderValue::from_str("access_token").unwrap();
            let token = req.headers().get("access_token");
            if token.clone().is_none() {
                Err(error::ErrorUnauthorized("无效token"))
            }else {
                 let token = token.unwrap();
                if req.path() != "/amin/index/login" {
                    if token.len() > 0 {
                        let x = token.as_bytes();

                        let result = String::from_utf8(Vec::from(x)).unwrap();
                        let redis = get_user_from_redis(&result)
                            .await
                            .expect("this is a user object");
                        req.extensions_mut().insert(redis);
                        Ok(svc.call(req).await?)
                    } else {
                        Err(error::ErrorUnauthorized("无效token"))
                    }
                } else {
                    Ok(svc.call(req).await?)
                }
            }
        })
    }
}
