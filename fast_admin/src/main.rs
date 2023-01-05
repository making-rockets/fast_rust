//extern crate crate;
#![allow(unused)]

use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

use crate::routers::menu_route;

mod base;
mod common;
mod config;
mod controller;
mod middleware;
mod models;
mod routers;
mod service;
mod storage;
mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::auth::Authorization)
            .wrap(middleware::handle_method::HandleMethod)
            .service(routers::index_route::index_routers())
            .service(routers::user_route::user_routes())
            .service(routers::menu_route::menu_routes())
    })
    .keep_alive(KeepAlive::Os)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("examples/basic/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
       //  tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}
