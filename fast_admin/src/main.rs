//extern crate crate;
#![allow(unused)]

use std::collections::{hash_map, HashMap};
use std::sync::Arc;

use actix_files::Files;
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::{routes, web, App, HttpResponse, HttpServer, Responder};

use chrono::naive::serde;
use lazy_static::lazy_static;
use mysql_async::Conn;
use once_cell::sync::Lazy;
use serde_json::{from_value, to_value, Value};
use tera::{try_get_value, Context, Error, Tera};
use tokio::sync::Mutex;

mod base;
mod common;
mod config;
mod controller;
mod middleware;
mod models;
mod router;
mod service;
mod storage;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //注册日志
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            //.app_data(web::Data::new(tera.clone()))
            //.app_data(web::Data::new(conn))
            //.wrap(middleware::auth::Authorization)
            .wrap(middleware::handle_method::HandleMethod)
            .service(router::index_router())
            .service(router::menu_router())
            .service(router::student_router())
            .service(router::user_router())
            .service(
                Files::new("/assets", "fast_admin/src/templates/teacher/assets")
                    .show_files_listing(),
            ) //静态文件
    })
        .keep_alive(KeepAlive::Os)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}

pub static GLOBAL_CONN_ONCE_CELL: Lazy<mysql_async::Pool> = Lazy::new(|| {
    let pool: mysql_async::Pool = mysql_async::Pool::new("mysql://root:root123@127.0.0.1:3306/test");
    pool
});

lazy_static! {
    pub static ref  GLOBAL_CONN_LAZY_STATIC: mysql_async::Pool =  mysql_async::Pool::new("mysql://root:root123@127.0.0.1:3306/test");
}


lazy_static! {
    //注册tera
  pub static ref  GLOBAL_TERA:Tera = match Tera::new("fast_admin/src/templates/teacher/*.html") {
        Ok(mut tera) => {
             tera.full_reload().map_err(|e| println!("{:?}", e));
             tera
        },
        Err(e) => {
            println!("错误{:?}", e);
            std::process::exit(1);
        }
    };


}