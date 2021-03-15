use crate::service::user_service::UserService;
use actix_web::web::Form;
use actix_web::{HttpResponse, Responder};
use actix_web::{get, post, HttpRequest};
use fast_common::models::user::UserLoginVo;
use fast_common::utils::captcha_util;
use std::ops::DerefMut;
use actix_http::Response;
use fast_common::common::api_result::{Api, GlobalError};

#[get("/")]
pub async fn index(request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("hello,actix-web")
}

#[get("/send_reg_code")]
pub async fn push_reg_code(user_name: String, _password: String, _code: String, _request: HttpRequest) -> impl Responder {
    captcha_util::BarCode::captcha().await
}


#[post("/login")]
pub async fn login(user: Form<UserLoginVo>) -> Response {
    let result = UserService::login(user.into_inner()).await;
    Api::from_rbatis_result(&result).await.to_response_of_json().await
}
