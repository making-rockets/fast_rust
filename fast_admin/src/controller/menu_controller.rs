use crate::common::api_result::Api;
use crate::models::menu::{Menu, MenuVo};
 
use actix_web::web::{Data, Json};
use actix_web::{  post,  };
use actix_web::{HttpRequest, HttpResponse};
use sqlx::{Pool, Sqlite};

#[post("/add_menu")]
pub async fn add_menu(arg: Json<Menu>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result  =  Menu::add_menu(arg.0, &pool).await;
    Api::from_any_result(result).await.to_response_of_json().await
}

#[post("/list_menu")]
pub async fn list_menu(arg: Json<MenuVo>, _req: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let result = Menu::get_menu_by_user_id(arg.user_id.unwrap(), &pool).await;
    Api::from_any_result(result).await.to_response_of_json().await
}

#[post("/edit_menu")]
pub async fn edit_menu(_arg: Json<Menu>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    todo!()
}

#[post("/delete_menu")]
pub async fn delete_menu(arg: Json<Menu>, _request: HttpRequest, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    todo!()
}

#[post("/page_menu")]
pub async fn page_menu(arg: Json<Menu>) -> HttpResponse {
    todo!()
}