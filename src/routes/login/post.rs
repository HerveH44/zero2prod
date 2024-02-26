use actix_web::{post, web, HttpResponse, Responder};
use reqwest::header::LOCATION;
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    username: String,
    password: Secret<String>,
}

#[post("/login")]
pub async fn login(_form: web::Form<LoginFormData>) -> impl Responder {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
