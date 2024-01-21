use actix_web::{test, App};

use zero2prod::routes::health_check;

#[actix_web::test]
async fn health_check_works() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::get().uri("/health_check").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
