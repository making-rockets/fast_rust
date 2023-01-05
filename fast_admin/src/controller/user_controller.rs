use crate::common::api_result::Api;
use crate::models::user::{User, UserVo};
use crate::service::user_service::UserService;
use actix_web::web::Form;
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};

#[post("new")]
pub async fn new_user(arg: Form<User>, _request: HttpRequest) -> HttpResponse {
    todo!()
}

#[get("list")]
pub async fn list(arg: Query<UserVo>, request: HttpRequest) -> HttpResponse {
    todo!()
}

#[put("update")]
pub async fn update(arg: Form<User>, _request: HttpRequest) -> HttpResponse {
    todo!()
}

#[delete("delete")]
pub async fn delete(arg: Form<User>) -> HttpResponse {
    todo!()
}
