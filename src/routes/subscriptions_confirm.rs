use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::SubscriptionToken;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(pool, parameters))]
#[get("/subscriptions/confirm")]
pub async fn confirm(
    pool: web::Data<PgPool>,
    parameters: web::Query<Parameters>,
) -> impl Responder {
    let subscription_token = match SubscriptionToken::parse(&parameters.subscription_token) {
        Ok(subscription_token) => subscription_token,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };
    let id = match get_subscriber_id_from_token(&pool, subscription_token.as_ref()).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match id {
        None => HttpResponse::Unauthorized().finish(),
        Some(subscriber_id) => {
            if confirm_subscriber(&pool, subscriber_id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
    }
}

#[instrument(
    name = "Mark subscriber as confirmed"
    skip(pool, subscriber_id)
)]
async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[instrument(
    name = "Get subscriber_id from token"
    skip(pool, subscription_token)
)]
async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let record = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        subscription_token,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(record.map(|r| r.subscriber_id))
}
