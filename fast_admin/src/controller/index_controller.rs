use crate::service::user_service::UserService;
use actix_web::web::{Form, Query};
use actix_web::{HttpResponse};
use actix_web::{get, post, HttpRequest};
use fast_common::models::user::{UserLoginVo};
use fast_common::utils::captcha_util;
use actix_http::{Response};
use fast_common::common::api_result::{Api, GlobalError};
use fast_common::utils::redis_util::RedisUtil;

#[get("/")]
pub async fn index(request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("hello,actix-web")
}

#[get("/send_reg_code")]
pub async fn push_reg_code(user_name: Query<UserLoginVo>, _request: HttpRequest) -> HttpResponse {
    match user_name.into_inner().user_name {
        Some(user_name) => {
            RedisUtil::get_redis_util().await.set_string_expire(&user_name,&"s".to_string(),200);
            captcha_util::BarCode::captcha().await
        }
        None => {

            Api::<()>::from_result(Err(GlobalError::from("user_name is none ".to_string()))).await.to_response_of_json().await
        }
    }
}


#[post("/login")]
pub async fn login(user: Form<UserLoginVo>) -> Response {
    let result = UserService::login(user.into_inner()).await;
    Api::from_rbatis_result(&result).await.to_response_of_json().await
}
