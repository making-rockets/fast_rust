use crate::service::menu_service::MenuService;
use actix_web::web::{Form, FormConfig, Json};
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};
use fast_common::common::api_result::ApiResult;
use fast_common::models::menu::{Menu, MenuVo};

#[post("/admin/menu/new")]
pub async fn new_user(arg: Form<Menu>, _request: HttpRequest) -> HttpResponse {
    let result = MenuService::add(arg.0).await;
    return ApiResult::from_result(&result).await.resp().await;
}

#[get("/admin/menu/list")]
pub async fn list(arg: Query<MenuVo>, _req: HttpRequest) -> HttpResponse {
    let list = MenuService::list(arg.0).await;
    return ApiResult::from_result(&list).await.resp().await;
}

#[put("/admin/menu/update")]
pub async fn update(arg: Form<Menu>, _request: HttpRequest) -> HttpResponse {
    let result = MenuService::update(arg.0).await;
    return ApiResult::from_result(&result).await.resp().await;
}

#[delete("/admin/menu/delete")]
pub async fn delete(arg: Form<Menu>) -> HttpResponse {
    let result = MenuService::delete(arg.0).await;
    return ApiResult::from_result(&result).await.resp().await;
}
