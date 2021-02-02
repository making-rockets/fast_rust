use actix_web::web::{Data, Form, Json, Query};
use actix_web::{HttpResponse, HttpRequest};
use crate::service::user_service::UserService;
use fast_common::common::api_result::ApiResult;
use fast_common::models::domain::user::User;
use fast_common::models::domain::user::UserRequest;
use fast_common::utils::crypt_util::decrypt_string;
use fast_common::utils::crypt_util::encrypt;
use fast_common::utils::redis_util;
use redis_tang::{Pool, RedisManager};
use log::kv::Source;
use fast_common::utils::redis_util::RedisUtil;

pub struct UserController;

impl UserController {
    pub async fn index() -> HttpResponse {
        HttpResponse::Ok().body("hello,world")
    }
    pub async fn new_user(arg: Json<UserRequest>,request:HttpRequest) -> HttpResponse {
        let result = UserService::add(arg.0).await;

        return ApiResult::from_result(&result).resp();
    }
    pub async fn list(arg: Query<UserRequest>, req: HttpRequest) -> HttpResponse {
        let redis_result = RedisUtil::set("a".to_string(), "b".to_string()).await;
        println!("{:?},这是redis返回的结果", redis_result.unwrap());
        let list = UserService::list(arg.0).await;
        return ApiResult::from_result(&list).resp();
    }
}
