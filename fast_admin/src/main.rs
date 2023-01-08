//extern crate crate;
#![allow(unused)]

use std::collections::{hash_map, HashMap};

use actix_files::Files;
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::{routes, web, App, HttpResponse, HttpServer, Responder};

use chrono::naive::serde;
use lazy_static::lazy_static;
use serde_json::{from_value, to_value, Value};
use tera::{try_get_value, Context, Error, Tera};

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
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
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
        // tera.register_function("context_template()", context_path);
        tera.full_reload().map_err(|e| println!("{:?}", e));

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera))
            //.wrap(middleware::auth::Authorization)
            .wrap(middleware::handle_method::HandleMethod)
            .service( router::routers())
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
