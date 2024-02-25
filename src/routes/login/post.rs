use actix_web::{post, HttpResponse, Responder};
use reqwest::header::LOCATION;

#[post("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
