use crate::controller::user_controller as user;
use actix_web::dev::HttpServiceFactory;
use actix_web::web;

pub(crate) fn user_routes() -> impl HttpServiceFactory {
    web::scope("/admin/user")
        .service(user::new_user)
        .service(user::update)
        .service(user::delete)
        .service(user::list)
}
