use crate::service::user_service::UserService;
use actix_web::web::{Form, Query};
use actix_web::{HttpResponse};
use actix_web::{get, post, HttpRequest};
use fast_common::models::user::{UserLoginVo};
use fast_common::utils::captcha_util;
use actix_http::{Response};
use fast_common::common::api_result::{Api, GlobalError};
use fast_common::utils::redis_util::RedisUtil;
use fast_common::utils::captcha_util::BarCode;


#[get("/")]
pub async fn index(request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("hello,actix-web")
}

#[get("/send_reg_code")]
pub async fn push_reg_code(user_name: Query<UserLoginVo>, _request: HttpRequest) -> HttpResponse {
    match user_name.into_inner().user_name {
        Some(user_name) => {
            let bar_code = BarCode::new(Some(user_name.clone()), None).await;
            let result = bar_code.captcha().await;
            match result {
                Some(png_code) => {

                    RedisUtil::get_redis_util().await.set_string_expire(&user_name,  &png_code.1.iter().collect(), 60).await;
                    bar_code.to_response(png_code.0).await
                }
                None => {
                    Api::<()>::from_result(Err(GlobalError::from("生成验证码错误".to_owned()))).await.to_response_of_json().await
                }
            }
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
