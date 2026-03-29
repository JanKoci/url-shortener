use crate::errors::AppError;
use crate::utils::generate_code;
use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
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
    let code = generate_code();
    sqlx::query!(
        "INSERT INTO urls (short_code, original_url) VALUES ($1, $2)",
        code,
        body.url
    )
    .execute(&state.db)
    .await?;

    Ok(Json(ShortenResponse { short_code: code }))
}
