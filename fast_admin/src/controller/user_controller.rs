use crate::common::api_result::Api;
use crate::models::user::{DeleteUsers, ReqPageUserVo, User};

use crate::models::user;
use actix_web::post;
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse};
use sqlx::{Pool, Sqlite};

#[post("get_by_user_id")]
pub async fn get_by_user_id(
    arg: Json<User>,
    _request: HttpRequest,
    pool: Data<Pool<Sqlite>>,
) -> HttpResponse {
    if arg.user_id.is_none() {
        let err = Api::<()>::error("userId 为空".to_string())
            .await
            .to_response_of_json()
            .await;
        return err;
    };

    let user = user::User::get_user_by_user_id(arg.user_id.unwrap(), &pool).await;
    Api::from_any_result(user).await.to_response_of_json().await
}

#[post("/add_user")]
pub async fn add_user(
    arg: Json<User>,
    _request: HttpRequest,
    pool: Data<Pool<Sqlite>>,
) -> HttpResponse {
    let result = user::User::add_user(arg.0, &pool).await;
    Api::from_any_result(result)
        .await
        .to_response_of_json()
        .await
}

#[post("/list_user")]
pub async fn list_user(
    user: Json<ReqPageUserVo>,
    _request: HttpRequest,
    pool: Data<Pool<Sqlite>>,
) -> HttpResponse {
    let result = user::User::user_list(user.into_inner(), &pool).await;
    Api::from_any_result(result)
        .await
        .to_response_of_json()
        .await
}

#[post("edit_user")]
pub async fn edit_user(
    arg: Json<User>,
    _request: HttpRequest,
    pool: Data<Pool<Sqlite>>,
) -> HttpResponse {
    let result = user::User::edit_user(arg.0, &pool).await;
    Api::from_any_result(result)
        .await
        .to_response_of_json()
        .await
}

#[post("delete_user")]
pub async fn delete_user(
    arg: Json<User>,
    _request: HttpRequest,
    pool: Data<Pool<Sqlite>>,
) -> HttpResponse {
    let result = user::User::delete_user(arg.user_id.unwrap(), &pool).await;
    Api::from_any_result(result)
        .await
        .to_response_of_json()
        .await
}
#[post("delete_users")]
pub async fn delete_users(
    arg: Json<DeleteUsers>,
    _request: HttpRequest,
    pool: Data<Pool<Sqlite>>,
) -> HttpResponse {
    let result = user::User::delete_users(arg.0, &pool).await;
    Api::from_any_result(result)
        .await
        .to_response_of_json()
        .await
}

#[post("page_user")]
pub async fn page_user(
    arg: Json<ReqPageUserVo>,
    _request: HttpRequest,
    pool: Data<Pool<Sqlite>>,
) -> HttpResponse {
    let result = user::User::user_page(arg.0, &pool).await;
    Api::from_any_result(result)
        .await
        .to_response_of_json()
        .await
}
