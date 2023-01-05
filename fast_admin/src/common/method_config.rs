#[macro_export]
macro_rules! get {
    ($path:expr,$func:expr) => {{
        actix_web::web::resource($path).route(actix_web::web::get().to($func))
    }};
}
#[macro_export]
macro_rules! post {
    ($path:expr,$func:expr) => {{
        actix_web::web::resource($path).route(actix_web::web::post().to($func))
    }};
}
#[macro_export]
macro_rules! delete {
    ($path:expr,$func:expr) => {{
        actix_web::web::resource($path).route(actix_web::web::delete().to($func))
    }};
}
#[macro_export]
macro_rules! put {
    ($path:expr,$func:expr) => {{
        actix_web::web::resource($path).route(actix_web::web::put().to($func))
    }};
}
