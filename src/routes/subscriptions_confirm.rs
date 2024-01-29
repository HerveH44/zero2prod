use std::fmt::Debug;

use actix_web::{get, web, HttpResponse, ResponseError};
use anyhow::Context;
use reqwest::StatusCode;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::SubscriptionToken;

use super::error_chain_fmt;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[derive(thiserror::Error)]
pub enum ConfirmationError {
    #[error("{0}")]
    ValidationError(String),
    #[error("There is no subscriber associated with the given token")]
    UnknownToken,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for ConfirmationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ConfirmationError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ConfirmationError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ConfirmationError::UnknownToken => StatusCode::UNAUTHORIZED,
            ConfirmationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(pool, parameters))]
#[get("/subscriptions/confirm")]
pub async fn confirm(
    pool: web::Data<PgPool>,
    parameters: web::Query<Parameters>,
) -> Result<HttpResponse, ConfirmationError> {
    let subscription_token = SubscriptionToken::parse(&parameters.subscription_token)
        .map_err(ConfirmationError::ValidationError)?;
    let subscriber_id = get_subscriber_id_from_token(&pool, subscription_token.as_ref())
        .await
        .context("Failed to retrieve susbcriber id from the provided token.")?
        .ok_or(ConfirmationError::UnknownToken)?;

    confirm_subscriber(&pool, subscriber_id)
        .await
        .context("Failed to update subscriber status to `confirmed`.")?;

    Ok(HttpResponse::Ok().finish())
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
