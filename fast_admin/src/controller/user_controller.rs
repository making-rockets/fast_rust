use crate::common::api_result::Api;
use crate::models::user::{User};
use crate::service::user_service::UserService;
use actix_web::web::{Data, Form, Json};
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};
use sqlx::{Pool, Sqlite};
use crate::models::user;

#[post("new")]
pub async fn new_user(arg: Form<User>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::add_user(arg.0, &pool).await;
    return Api::from_any_result(result).await.to_response_of_json().await;
}

#[get("list")]
pub async fn list(arg: Query<User>, request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::user_list(arg.0, &pool).await;
    return Api::from_any_result(result).await.to_response_of_json().await;
}

#[put("update")]
pub async fn update(arg: Form<User>, _request: HttpRequest) -> HttpResponse {
    todo!()
}

#[delete("delete")]
pub async fn delete(arg: Form<User>) -> HttpResponse {
    todo!()
}

#[get("page")]
pub async fn page(arg: Json<User>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = user::User::user_page(arg.0, 1, 10, &pool).await;
    return Api::from_any_result(result).await.to_response_of_json().await;
}
