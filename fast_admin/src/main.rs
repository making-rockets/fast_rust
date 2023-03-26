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
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::{routes, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use async_once::AsyncOnce;


use chrono::naive::serde;
use futures_util::TryFutureExt;
use lazy_static::lazy_static;


use serde_json::{from_value, to_value, Value};
use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use tera::{try_get_value, Context, Error, Tera};

use tokio::sync::Mutex;
use tracing::log;
use tracing::log::{LevelFilter, Log};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


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
    std::env::set_var("RUST_LOG", "actix_web=DEBUG");
    env_logger::init();



    let current_path_buf = env::current_dir().unwrap();
    let current_path = current_path_buf.as_path().to_str().unwrap();
    let mut sqlite_connect_options = SqliteConnectOptions::from_str(&format!("sqlite://{}/db.sqlite", current_path)).expect("打不开");
    sqlite_connect_options = sqlite_connect_options.journal_mode(SqliteJournalMode::Wal);
    sqlite_connect_options.log_statements(LevelFilter::Debug);

    sqlite_connect_options.connect().await.unwrap_or_else(|_| std::process::exit(1));

    let sql_pool = SqlitePoolOptions::new().connect_with(sqlite_connect_options.clone()).await.unwrap();


    HttpServer::new(move || {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(Logger::default())
            .app_data(Data::new(sql_pool.clone()))
            //.wrap(middleware::auth::Authorization)
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

  // pub static ref GLOBAL_SQLX:AsyncOnce<Pool<Sqlite>> = {
  //
  //      let t =  AsyncOnce::new(async {
  //           sqlx_pool().await
  //       });
  //       t
  // } ;




}


// pub async fn sqlx_pool() -> Pool<Sqlite> {
//     let mut sqlite_connect_options = SqliteConnectOptions::from_str("sqlite://D:\\project\\rust\\fast_rust\\db.sqlite").expect("打不开");
//     sqlite_connect_options = sqlite_connect_options.journal_mode(SqliteJournalMode::Wal);
//     sqlite_connect_options.log_statements(LevelFilter::Debug);
//     sqlite_connect_options.connect().await.unwrap_or_else(|_| std::process::exit(1));
//     let sql_pool = SqlitePoolOptions::new().connect_with(sqlite_connect_options.clone()).await.unwrap();
//     sql_pool
// }



