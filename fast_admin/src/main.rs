//extern crate crate;
#![allow(unused)]

use std::collections::{hash_map, HashMap};
use std::env;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr;
use std::sync::Arc;

use actix_files::Files;
use actix_http::header::CONTENT_TYPE;
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::{routes, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use async_once::AsyncOnce;


use chrono::naive::serde;
use futures_util::TryFutureExt;
use lazy_static::lazy_static;
use log::{info, LevelFilter, log};
use log4rs::Config;


use serde_json::{from_value, to_value, Value};
use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use tera::{try_get_value, Context, Error, Tera};


use tracing_actix_web::TracingLogger;


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


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parent_path = config_path.parent().unwrap().to_str().unwrap();
    //注册日志
    std::env::set_var("RUST_LOG", "actix_web=DEBUG");

    log4rs::init_file(format!("{}/{}",&parent_path,"config.yaml"), Default::default()).unwrap();


    let mut sqlite_connect_options = SqliteConnectOptions::from_str(&format!("sqlite://{}/db.sqlite", parent_path)).unwrap();
    sqlite_connect_options = sqlite_connect_options.journal_mode(SqliteJournalMode::Wal);
    sqlite_connect_options.log_statements(LevelFilter::Debug);

    sqlite_connect_options.connect().await.unwrap_or_else(|_| std::process::exit(1));

    let sql_pool = SqlitePoolOptions::new().connect_with(sqlite_connect_options.clone()).await.unwrap();


    HttpServer::new(move || {
        App::new()

            .wrap(Logger::new(r#"远程地址={%a} "请求方式={%r}" 状态码={%s} 相应数据大小={%b}}  useragent ={"%{User-Agent}i"}"#))
            // .wrap(TracingLogger::default())
            .app_data(Data::new(sql_pool.clone()))
            .wrap(middleware::auth::Authorization)
            //.wrap(middleware::handle_method::HandleMethod)
            .wrap(actix_web::middleware::Compress::default())
            .service(router::index_router())
            .service(router::menu_router())
            .service(router::student_router())
            .service(router::user_router())
            .service(Files::new("/assets", "src/templates/teacher/assets").show_files_listing())
        //静态文件
    })
        .keep_alive(KeepAlive::Os)
        .bind("0.0.0.0:3000")?
        .workers(2)
        .run()
        .await
}

lazy_static! {
    //注册tera
  pub static ref  GLOBAL_TERA:Tera = match Tera::new("src/templates/teacher/*.html") {
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
