use actix_web::get;
use actix_web::HttpResponse;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("hello,world")
}
