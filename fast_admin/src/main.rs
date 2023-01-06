//extern crate crate;
#![allow(unused)]

use std::collections::HashMap;

use crate::routers::menu_route;
use actix_files::Files;
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use serde_json::{to_value, Value};
use tera::{try_get_value, Context, Tera};

mod base;
mod common;
mod config;
mod controller;
mod middleware;
mod models;
mod routers;
mod service;
mod storage;
mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(move || {
        //注册actix-files;

        //注册tera
        let mut tera = match Tera::new("fast_admin/src/templates/teacher/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("错误{:?}", e);
                std::process::exit(1);
            }
        };
        tera.full_reload().map_err(|e| println!("{:?}", e));
        let mut context = Context::new();
        context.insert("context_path", "http://127.0.0.1:3000");
        context.insert("website", "hello,world");
        tera.render_str("context_path", &context);
        //tera.register_function("list_new_articles",funs::article::make_list_new_articles(db_util::POOL.clone()));
        //tera.register_function("list_recommend_articles",funs::article::make_list_recommend_articles(db_util::POOL.clone()));

        App::new()
            .app_data(web::Data::new(tera))
            .wrap(Logger::default())
            //.wrap(middleware::auth::Authorization)
            .wrap(middleware::handle_method::HandleMethod)
            .service(routers::index_route::index_routers())
            .service(routers::user_route::user_routes())
            .service(routers::menu_route::menu_routes())
            .service(Files::new(
                "assets",
                "fast_admin/src/templates/teacher/assets",
            )) //静态文件
    })
    .keep_alive(KeepAlive::Os)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(s).unwrap())
}
