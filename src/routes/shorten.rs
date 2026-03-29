use crate::errors::AppError;
use crate::utils::generate_code;
use crate::AppState;
use axum::{extract::State, Json};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    pub expires_in_seconds: Option<i64>,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_code: String,
}

pub async fn shorten_handler(
    State(state): State<AppState>,
    Json(body): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
    if Url::parse(&body.url).is_err() {
        return Err(AppError::BadRequest("Invalid URL".to_string()));
    }
    let expires_at = body
        .expires_in_seconds
        .map(|seconds| Utc::now() + Duration::seconds(seconds));
    let code = generate_code();
    sqlx::query!(
        "INSERT INTO urls (short_code, original_url, expires_at) VALUES ($1, $2, $3)",
        code,
        body.url,
        expires_at
    )
    .execute(&state.db)
    .await?;

    Ok(Json(ShortenResponse { short_code: code }))
}
