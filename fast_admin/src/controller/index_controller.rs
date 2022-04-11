use std::collections::HashMap;

use actix_http::Response;
use actix_web::{HttpMessage, HttpResponse, web};
use actix_web::{get, HttpRequest, post};
use actix_web::http::header;
use actix_web::web::{Form, Header, Query};

use fast_common::common::api_result::{Api, GlobalError};
use fast_common::models::user::UserLoginVo;
use fast_common::utils::captcha_util::BarCode;
use fast_common::utils::redis_util::{REDIS_UTIL, RedisUtil};

use crate::service::user_service::UserService;

#[get("/send_reg_code")]
pub async fn push_reg_code(user_name: Query<UserLoginVo>) -> HttpResponse {
    match user_name.into_inner().user_name {
        Some(user_name) => {
            let bar_code = BarCode::new(Some(user_name.clone()), None).await;
            let result = bar_code.captcha().await;
            match result {
                Some(png_code) => {
                    let code_vec = png_code.1.iter().collect::<String>();
                    REDIS_UTIL.set_by_nx(&user_name, code_vec).await;
                    bar_code.to_response(png_code.0).await
                }
                None => {
                    Api::<()>::error("生成验证码错误".to_owned()).await.to_response_of_json().await
                }
            }
        }
        None => {
            Api::<()>::error("user_name is none ".to_owned()).await.to_response_of_json().await
        }
    }
}


#[post("/login")]
pub async fn login(user: Form<UserLoginVo>) -> HttpResponse {
    println!("{:?}",user);
    let result = UserService::login(user.into_inner()).await;
    Api::from_any_result(result).await.to_response_of_json().await
}




#[get("/logout")]
pub async fn logout(request: HttpRequest) -> HttpResponse {
    let header = request.headers().get("authorization");
    match header {
        None => { Api::<()>::error(String::from("未登录")).await.to_response_of_json().await }
        Some(access_token) => {
            REDIS_UTIL.delete(access_token.to_str().unwrap()).await;
            Api::<()>::success().await.to_response_of_json().await
        }
    }
}


