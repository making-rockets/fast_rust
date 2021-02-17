use crate::controller::index_controller as index;
use actix_web::dev::HttpServiceFactory;
use actix_web::web;

pub(crate) fn index_routers() -> impl HttpServiceFactory {
    web::scope("/admin/index/")
        .service(index::index)
        .service(index::push_reg_code)
        .service(index::login)
}
