use crate::service::menu_service::MenuService;
use actix_web::web::Form;
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};
use fast_common::common::api_result::{Api};
use fast_common::models::menu::{Menu, MenuVo};

#[post("/new")]
pub async fn new_user(arg: Form<Menu>, _request: HttpRequest) -> HttpResponse {
    let result = MenuService::add(arg.into_inner()).await;
    return Api::from_rbatis_result(result).await.to_response_of_json().await;
}

#[get("/list")]
pub async fn list(arg: Query<MenuVo>, _req: HttpRequest) -> HttpResponse {
    let result = MenuService::list(arg.into_inner()).await;
    return Api::from_rbatis_result(result).await.to_response_of_json().await;
}

#[put("/update")]
pub async fn update(arg: Form<Menu>, _request: HttpRequest) -> HttpResponse {
    let result = MenuService::update(arg.into_inner()).await;
    return Api::from_rbatis_result(result).await.to_response_of_json().await;
}

#[delete("/delete")]
pub async fn delete(arg: Form<Menu>) -> HttpResponse {
    let result = MenuService::delete(arg.into_inner()).await;
    return Api::from_rbatis_result(result).await.to_response_of_json().await;
}
