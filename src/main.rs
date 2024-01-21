use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    domain::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender_email address.");
    let base_url = configuration.email_client.url().expect("Invalid base_url");
    let timeout = configuration.email_client.timeout();
    let authorization_token = configuration.email_client.authorization_token;
    let email_client = EmailClient::new(base_url, sender_email, authorization_token, timeout);

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)?.await
}
