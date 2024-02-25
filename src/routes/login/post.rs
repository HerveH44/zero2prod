use actix_web::http::header::ContentType;
use actix_web::{post, HttpResponse, Responder};

#[post("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}
