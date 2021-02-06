extern crate fast_common;

use fast_common::common::orm_config::RB;
use fast_common::middleware;

use fast_common::common::orm_config::InitDb;


use actix_web::{App, HttpServer};
use env_logger::{Builder, Env};
use log::LevelFilter;

mod controller;
mod routers;
mod service;

fn init_logger() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
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

    let db_url = std::env::var("db_url")
        .unwrap_or_else(|_| String::from("mysql://root:root@39.97.225.5:3306/go"));
    RB.link_opt(db_url.as_str(), &InitDb::db_option())
        .await
        .unwrap();
    init_logger();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::auth::Auth)
            .service(controller::index_controller::index)
            .service(routers::user_route::user_routes())
            .service(routers::menu_route::menu_routes())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
