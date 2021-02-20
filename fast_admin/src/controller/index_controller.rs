use crate::service::user_service;
use crate::service::user_service::UserService;
use actix_web::web::Form;
use actix_web::HttpResponse;
use actix_web::{get, post, HttpRequest};
use fast_common::common::api_result::ApiResult;
use fast_common::models::user::UserLoginVo;
use std::ops::DerefMut;

#[get("/")]
pub async fn index(request: HttpRequest) -> HttpResponse {
    let mut extensions = request.extensions_mut();
    let x = extensions.deref_mut();
    let option = x.get_mut::<String>();
    println!("{:?}", option);

    HttpResponse::Ok().body("hello,actix-web")
}

#[get("/send_reg_code")]
pub async fn push_reg_code(
    user_name: String,
    password: String,
    code: String,
    request: HttpRequest,
) -> HttpResponse {
    println!("{}", user_name);
    return HttpResponse::Ok().body("hello,world");
}

#[post("/login")]
pub async fn login(user: Form<UserLoginVo>) -> HttpResponse {
    let result = UserService::login(user.0).await;
    return ApiResult::from_result(&result).await.resp().await;
}
