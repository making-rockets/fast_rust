use crate::service::user_service::UserService;
use actix_web::web::Form;
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};
use fast_common::common::api_result::{Api};
use fast_common::models::user::{User, UserVo};


#[post("new")]
pub async fn new_user(arg: Form<User>, _request: HttpRequest) -> HttpResponse {
    let result = UserService::add(arg.into_inner()).await;
    return Api::from_any_result(result).await.to_response_of_json().await;
}

#[get("list")]
pub async fn list(arg: Query<UserVo>, request: HttpRequest) -> HttpResponse {
    let list = UserService::list(arg.into_inner()).await;
    return Api::from_any_result(list).await.to_response_of_json().await;
}

#[put("update")]
pub async fn update(arg: Form<User>, _request: HttpRequest) -> HttpResponse {
    let result = UserService::update(arg.into_inner()).await;
    return Api::from_any_result(result).await.to_response_of_json().await;
}

#[delete("delete")]
pub async fn delete(arg: Form<User>) -> HttpResponse {
    let result = UserService::delete(arg.into_inner()).await;
    return Api::from_any_result(result).await.to_response_of_json().await;
}
