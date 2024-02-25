use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn homepage() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("home.html"))
}
