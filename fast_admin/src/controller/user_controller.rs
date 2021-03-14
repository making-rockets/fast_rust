use crate::service::user_service::UserService;
use actix_web::web::Form;
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};
use fast_common::common::api_result::{Api};
use fast_common::models::user::{User, UserVo};
use std::ops::DerefMut;
/*
#[post("/new")]
pub async fn new_user(arg: Form<User>, _request: HttpRequest) -> HttpResponse {
    let result = UserService::add(arg.0).await;
    return ApiResult::from_result(&result).await.resp().await;
}

#[get("/list")]
pub async fn list(arg: Query<UserVo>, request: HttpRequest) -> HttpResponse {

    let list = UserService::list(arg.0).await;
    return Api::from(list).await.to_response_of_json().await;
}

#[put("/update")]
pub async fn update(arg: Form<User>, _request: HttpRequest) -> HttpResponse {
    let result = UserService::update(arg.0).await;
    return ApiResult::from_result(&result).await.resp().await;
}

#[delete("/delete")]
pub async fn delete(arg: Form<User>) -> HttpResponse {
    let result = UserService::delete(arg.0).await;
    return ApiResult::from_result(&result).await.resp().await;
}*/
