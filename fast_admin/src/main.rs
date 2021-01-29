#[macro_use]
extern crate fast_common;

use fast_common::common::orm_config::RB;
use fast_common::common;
use fast_common::middleware;
use fast_common::models;


use fast_common::common::orm_config::InitDb;
use fast_common::models::domain::user::User;

use env_logger::{Builder, Env};
use log::LevelFilter;
use actix_web::{App, HttpServer};
use actix_http::body::MessageBody;
mod service;
mod controller;
use crate::controller::user_controller::UserController;

fn init_logger() {
    let env = Env::default().filter_or("MY_LOG_LEVEL","trace")
        .write_style_or("MY_LOG_STYLE","always");
    Builder::from_env(env).filter_level(LevelFilter::Debug)
        .format_level(true)
        .format_timestamp_micros()
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    RB.link_opt("mysql://root:root@localhost:3306/go", &InitDb::db_option()).await.unwrap();
    init_logger();


    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::auth::Auth)
            .service(get!("/", UserController::index))
            .service(post!("/admin/user/new", UserController::new_user))
            .service(get!("/admin/user/list", UserController::list))
    }).workers(1)
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
