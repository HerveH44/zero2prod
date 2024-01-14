#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use actix_web::{test, web, App};
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use uuid::Uuid;
    use zero2prod::{
        configuration::{get_configuration, DatabaseSettings},
        routes::{health_check::health_check, subscriptions::subscribe},
    };

    #[actix_web::test]
    async fn health_check_works() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = Uuid::new_v4().to_string();
        let connection_pool = configure_database(&configuration.database).await;
        let db_pool = web::Data::new(connection_pool.clone());
        let app = test::init_service(App::new().service(subscribe).app_data(db_pool.clone())).await;

        let body = HashMap::from([("name", "le guin"), ("email", "ursula_le_guin@gmail.com")]);

        let req = test::TestRequest::post()
            .uri("/subscriptions")
            .set_form(body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&connection_pool)
            .await
            .expect("Failed to fetch saved subscription.");

        assert_eq!(saved.email, "ursula_le_guin@gmail.com");
        assert_eq!(saved.name, "le guin");
    }

    async fn configure_database(config: &DatabaseSettings) -> PgPool {
        // Create database
        let mut connection = PgConnection::connect_with(&config.without_db())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
            .await
            .expect("Failed to create database.");
        // Migrate database
        let connection_pool = PgPool::connect_with(config.with_db())
            .await
            .expect("Failed to connect to Postgres.");
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
        connection_pool
    }

    #[actix_web::test]
    async fn subscribe_returns_a_400_when_data_is_invalid() {
        let app = test::init_service(App::new().service(subscribe)).await;
        let test_cases = vec![
            (("name", "le%20guin"), "missing the email"),
            (("name", "{}}{}<>"), "bad name format"),
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
