use actix_web::{App, HttpServer};
use zero2prod::{health_check, subscribe};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
