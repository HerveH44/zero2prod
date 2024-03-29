use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse, Responder};

#[get("/login")]
pub async fn login_form() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}
