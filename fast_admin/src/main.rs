use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use actix_cors::Cors;
use actix_files::Files;
use actix_http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http, App, HttpServer};
use lazy_static::lazy_static;
use log::LevelFilter;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::ConnectOptions;

use tera::Tera;
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
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let parent_path = config_path.parent().unwrap().to_str().unwrap();
    //注册日志
    std::env::set_var("RUST_LOG", "actix_web=DEBUG");

    log4rs::init_file(
        format!("{}/{}", &parent_path, "config.yaml"),
        Default::default(),
    )
    .unwrap();

    let mut sqlite_connect_options =
        SqliteConnectOptions::from_str(&format!("sqlite://{}/db.sqlite", parent_path)).unwrap();
    sqlite_connect_options = sqlite_connect_options.journal_mode(SqliteJournalMode::Wal);
    sqlite_connect_options
        .clone()
        .log_statements(LevelFilter::Debug);

    sqlite_connect_options
        .connect()
        .await
        .unwrap_or_else(|_| std::process::exit(1));

    let sql_pool = SqlitePoolOptions::new()
        .connect_with(sqlite_connect_options.clone())
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::new(r#"%a  %r %s%b  useragent=%"#))
            // .wrap(TracingLogger::default())
            .app_data(Data::new(sql_pool.clone()))
            //.wrap(middleware::auth::Authorization)
            //.wrap(middleware::handle_method::HandleMethod)
            .wrap(actix_web::middleware::Compress::default())
            .wrap(cors)
            .service(router::index_router())
            .service(router::menu_router())
            .service(router::user_router())
        //.service(Files::new("/assets", "src/templates/teacher/assets").show_files_listing())
        //.service(Files::new("/fast_wasm/pkg", "../fast_wasm/pkg").show_files_listing())
        //静态文件fast_wasm\pkg
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
            tera.full_reload().expect("tera reload failed");
            tera
        },
        Err(e) => {
            println!("错误{:?}", e);
            std::process::exit(1);
        }
    };
}
