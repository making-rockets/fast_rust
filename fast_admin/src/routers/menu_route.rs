use crate::controller::menu_controller as menu;
use actix_web::dev::HttpServiceFactory;
use actix_web::web;

pub(crate) fn menu_routes() -> impl HttpServiceFactory {
    web::scope("/admin/menu")
        .service(menu::new_user)
        .service(menu::update)
        .service(menu::delete)
}
