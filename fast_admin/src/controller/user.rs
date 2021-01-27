use actix_web::web::{Data, Form, Json, Query};
use actix_web::HttpResponse;

use fast_common::common::api_result::ApiResult;
use fast_common::models::domain::user::User;
use fast_common::models::dto::user_dto::UserDTO;
use crate::service::user::UserService;

impl User {
    pub async fn index() -> HttpResponse {
        HttpResponse::Ok().body("hello,world")
    }
    pub async fn new_user(arg: Json<UserDTO>) -> HttpResponse {
        let result = UserService::add(&arg.0).await;
        return ApiResult::from_result(&result).resp();
    }
    pub async fn list(arg: Query<UserDTO>) -> HttpResponse {
        let list = UserService::list(&arg.0).await;
        return ApiResult::from_result(&list).resp();
    }
}
