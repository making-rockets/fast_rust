use crate::service::user_service::UserService;
use actix_web::{
    get, post,
    web::{Json, Query},
};
use actix_web::{HttpRequest, HttpResponse};
use fast_common::common::api_result::ApiResult;

use fast_common::models::domain::user::UserRequest;

use fast_common::utils::redis_util::RedisUtil;

#[post("/admin/user/new")]
pub async fn new_user(arg: Json<UserRequest>, _request: HttpRequest) -> HttpResponse {
    let result = UserService::add(arg.0).await;
    return ApiResult::from_result(&result).resp();
}

#[get("/admin/user/list")]
pub async fn list(arg: Query<UserRequest>, _req: HttpRequest) -> HttpResponse {
    let redis_result = RedisUtil::set("a".to_string(), "b".to_string()).await;
    println!("{:?},这是redis返回的结果", redis_result.unwrap());
    let list = UserService::list(arg.0).await;
    return ApiResult::from_result(&list).resp();
}
