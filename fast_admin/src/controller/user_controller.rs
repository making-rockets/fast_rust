use actix_web::web::{Data, Form, Json, Query};
use actix_web::{HttpResponse, HttpRequest};
use crate::service::user_service::UserService;
use fast_common::common::api_result::ApiResult;
use fast_common::models::domain::user::User;
use fast_common::models::domain::user::UserRequest;
use fast_common::utils::crypt_util::decrypt_string;
use fast_common::utils::crypt_util::encrypt;

pub struct UserController;

impl UserController {
    pub async fn index() -> HttpResponse {
        HttpResponse::Ok().body("hello,world")
    }
    pub async fn new_user(arg: Json<UserRequest>) -> HttpResponse {
        let result = UserService::add(arg.0).await;
        return ApiResult::from_result(&result).resp();
    }
    pub async fn list(arg: Query<UserRequest>,req:HttpRequest) -> HttpResponse {
        let list = UserService::list(arg.0).await;
        let stt = encrypt(&list.unwrap());
        let result = decrypt_string(stt.unwrap().as_str());
        //return ApiResult::from_result(&result).resp();
        println!("执行了{:?}", &result);
        return HttpResponse::Ok().body(result.unwrap());
    }
}
