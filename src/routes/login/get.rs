use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn login_form() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .finish()
}
