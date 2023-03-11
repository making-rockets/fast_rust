//extern crate crate;
#![allow(unused)]

use std::collections::{hash_map, HashMap};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr;
use std::sync::Arc;

use actix_files::Files;
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::{routes, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use async_static::async_static;


use chrono::naive::serde;
use futures_util::TryFutureExt;
use lazy_static::lazy_static;

use serde_json::{from_value, to_value, Value};
use sqlx::{Connection, ConnectOptions, Pool, Sqlite, SqliteConnection, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use tera::{try_get_value, Context, Error, Tera};

use tokio::sync::Mutex;
use tracing::log::{LevelFilter, Log};



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

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    println!("{:?}", config_path);

    let mut sqlite_connect_options = SqliteConnectOptions::from_str("sqlite://D:\\project\\rust\\fast_rust\\db.sqlite").expect("打不开");
    sqlite_connect_options = sqlite_connect_options.journal_mode(SqliteJournalMode::Wal);
    sqlite_connect_options.log_statements(LevelFilter::Debug);
    sqlite_connect_options.connect().await.unwrap_or_else(|_| std::process::exit(1));

    let sql_pool = SqlitePoolOptions::new().connect_with(sqlite_connect_options.clone()).await.unwrap();


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(sql_pool.clone()))
            //.wrap(middleware::auth::Authorization)
            .wrap(middleware::handle_method::HandleMethod)
            .service(router::index_router())
            .service(router::menu_router())
            .service(router::student_router())
            .service(router::user_router())
            .service(Files::new("/assets", "src/templates/teacher/assets").show_files_listing())
        //静态文件
    })
        .keep_alive(KeepAlive::Os)
        .bind("localhost:3000")?
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

// async_static! {
//   pub static ref pool:Pool<Sqlite> = {
//
//         let mut sqlite_connect_options = SqliteConnectOptions::new();
//         sqlite_connect_options = sqlite_connect_options.filename("sqlite://fast_rust.db");
//         sqlite_connect_options.log_statements(LevelFilter::Debug);
//         let sql_pool = SqlitePoolOptions::new().connect_with(sqlite_connect_options.clone()).await.unwrap();
//         return sql_pool;
//     };
// }



