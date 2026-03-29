use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use sqlx::types::chrono;
use crate::AppState;
use crate::errors::AppError;

#[derive(Serialize)]
pub struct StatsResponse {
    pub short_code: String,
    pub original_url: String,
    pub click_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn stats_handler(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<StatsResponse>, AppError> {
    let row = sqlx::query!("SELECT short_code, original_url, click_count, created_at FROM urls WHERE short_code = $1", code)
        .fetch_optional(&state.db)
        .await?;
    match row {
        Some(r) => Ok(Json(StatsResponse { short_code: r.short_code, original_url: r.original_url, click_count: r.click_count, created_at: r.created_at })),
        None => Err(AppError::NotFound),
    }
}