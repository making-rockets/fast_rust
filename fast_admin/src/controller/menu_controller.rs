use crate::common::api_result::Api;
use crate::models::menu::{Menu, MenuVo};
use crate::service::menu_service::MenuService;
use actix_web::web::Form;
use actix_web::{delete, get, post, put, web::Query};
use actix_web::{HttpRequest, HttpResponse};

#[post("/new")]
pub async fn new_user(arg: Form<Menu>, _request: HttpRequest) -> HttpResponse {
    todo!()
}

#[get("/list")]
pub async fn list(arg: Query<MenuVo>, _req: HttpRequest) -> HttpResponse {
    todo!()
}

#[put("/update")]
pub async fn update(arg: Form<Menu>, _request: HttpRequest) -> HttpResponse {
    todo!()
}

#[delete("/delete")]
pub async fn delete(arg: Form<Menu>) -> HttpResponse {
    todo!()
}
