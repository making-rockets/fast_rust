use actix_web::web::{Data, Form, Json, Query,};
use actix_web::HttpResponse;

use fast_common::common::api_result::ApiResult;
use fast_common::models::domain::user::User;
use fast_common::models::domain::user::UserRequest;
use crate:: service::user_service::UserService;

pub struct UserController;

impl UserController {

    pub async fn index() -> HttpResponse {
        HttpResponse::Ok().body("hello,world")
    }
    pub async fn new_user(arg: Json<UserRequest>) -> HttpResponse {
        let result = UserService::add(arg.0).await;
        return ApiResult::from_result(&result).resp();
    }
    pub async fn list(arg: Query<UserRequest>) -> HttpResponse {
        let list = UserService::list(arg.0).await;
        return ApiResult::from_result(&list).resp();
    }
}
