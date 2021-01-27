#[macro_use]
extern crate fast_common;

use fast_common::common;
use fast_common::middleware;
use fast_common::models;
use fast_common::utils;
use fast_common::common::base::DB_POOL_OPTIONS;
use fast_common::common::base::RB;
use fast_common::models::domain::user::User;

use fast_common::env_logger::{Builder, Env};
use log::LevelFilter;
use actix_web::{App, HttpServer};
use actix_http::body::MessageBody;
mod service;
mod controller;

fn init_logger() {
    let env = Env::default().filter("MY_LOG_LEVEL").write_style("MY_LOG_STYLE");
    Builder::from_env(env).filter_level(LevelFilter::max()).format_level(true).format_timestamp_nanos().init();
}






#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
   // RB.link_opt("mysql://root:root@localhost:3306/go",&DB_POOL_OPTIONS).await.unwrap();
    init_logger();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::auth::Auth)
            .service(get!("/", User::index))
            .service(post!("/user/new", User::new_user))
            .service(get!("/user/list", User::list))
    }).workers(4)
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
