use crate::common::api_result::Api;
use crate::models::user::{ReqPageUserVo, User};
use crate::service::user_service::UserService;
use actix_web::web::{Data, Form, Json};
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};
use sqlx::{Pool, Sqlite};
use crate::models::{Page, PageInfo, user};

#[post("/add_user")]
pub async fn add_user(arg: Json<User>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::add_user(arg.0, &pool).await;
    Api::from_any_result(result).await.to_response_of_json().await
}

#[post("/list_user")]
pub async fn list_user(user: Json<ReqPageUserVo>, request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::user_list(user.into_inner(), &pool).await;
    Api::from_any_result(result).await.to_response_of_json().await
}

#[post("edit_user")]
pub async fn edit_user(arg: Json<User>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::edit_user(arg.0, &pool).await;
    Api::from_any_result(result).await.to_response_of_json().await
}

#[post("delete_user")]
pub async fn delete_user(arg: Json<User>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::delete_user(arg.user_id.unwrap(), &pool).await;
    Api::from_any_result(result).await.to_response_of_json().await
}

#[post("page_user")]
pub async fn page_user(arg: Json<ReqPageUserVo>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::user_page(arg.0, &pool).await;
    Api::from_any_result(result).await.to_response_of_json().await
}
