//extern crate fast_common;
#![allow(unused)]

use fast_common::middleware;

use fast_common::common::orm_config::{InitDb, RB};

use actix_web::{App, HttpResponse, HttpServer, Responder, web, get};
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use env_logger::{Builder, Env};

use log::{debug, LevelFilter};
use fast_common::models::user::User;
use crate::routers::menu_route;

mod controller;
mod routers;
mod service;
mod storage;

fn init_logger() {
    let env = Env::default()
        .filter_or("RUST_LOG", "DEBUG")
        .write_style_or("MY_LOG_STYLE", "always");
    Builder::from_env(env)
        .filter_level(LevelFilter::Debug)
        .format_level(true)
        .format_timestamp_micros()
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    init_logger();

    RB.link_opt("mysql://root:root123@localhost:3306/test", InitDb::db_option()).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::auth::Authorization)
            .wrap(middleware::handle_method::HandleMethod)
            .service(routers::index_route::index_routers())
            .service(routers::user_route::user_routes())
            .service(routers::menu_route::menu_routes())
    }).keep_alive(KeepAlive::Os)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
