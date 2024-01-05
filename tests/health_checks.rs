#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use actix_web::{test, App};
    use zero2prod::routes::{health_check::health_check, subscriptions::subscribe};

    #[actix_web::test]
    async fn health_check_works() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let app = test::init_service(App::new().service(subscribe)).await;
        let body = HashMap::from([
            ("name", "le%20guin"),
            ("email", "ursula_le_guin%40gmail.com"),
        ]);

        let req = test::TestRequest::post()
            .uri("/subscriptions")
            .set_form(body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn subscribe_returns_a_400_when_data_is_missing() {
        let app = test::init_service(App::new().service(subscribe)).await;
        let test_cases = vec![
            (("name", "le%20guin"), "missing the email"),
            (("email", "ursula_le_guin%40gmail.com"), "missing the name"),
            (("", ""), "missing both the mail and the name"),
        ];
        for (invalid_params, error_message) in test_cases {
            let invalid_body = HashMap::from([invalid_params]);
            let req = test::TestRequest::post()
                .uri("/subscriptions")
                .set_form(invalid_body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(
                resp.status().as_u16(),
                400,
                "The API did not fail with 400 Bad Request when the payload was {}.",
                error_message
            );
        }
    }
}
