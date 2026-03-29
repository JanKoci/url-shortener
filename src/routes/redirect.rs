use crate::{errors::AppError, utils::is_expired, AppState};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use redis::AsyncCommands;

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut redis = state.redis.clone();
    // 1. check cache
    let cache_key = format!("redirect:{}", code);
    let cached: Option<String> = redis.get(&cache_key).await.unwrap_or(None);

    if let Some(url) = cached {
        tracing::info!(short_code = %code, "redirect cache hit");
        let db = state.db.clone();
        let code_clone = code.clone();
        tokio::spawn(async move {
            if let Err(e) = sqlx::query!(
                "UPDATE urls SET click_count = click_count + 1 WHERE short_code = $1",
                code_clone
            )
            .execute(&db)
            .await
            {
                tracing::error!(short_code = %code_clone, error = %e, "failed to update click count");
            }
        });
        return Ok(Redirect::to(&url).into_response());
    }

    // 2. cache miss — query DB
    let row = sqlx::query!(
        "SELECT original_url, expires_at FROM urls WHERE short_code = $1",
        code
    )
    .fetch_optional(&state.db)
    .await?;
    match row {
        Some(r) => {
            if is_expired(r.expires_at) {
                tracing::warn!(short_code = %code, "attempt to access expired link");
                return Err(AppError::Gone);
            }
            // store in Redis
            let ttl_seconds: u64 = r
                .expires_at
                .map(|d| (d - Utc::now()).num_seconds().max(1) as u64)
                .unwrap_or(3600); // default: cache for 1 hour

            let _: () = redis
                .set_ex(&cache_key, &r.original_url, ttl_seconds)
                .await
                .unwrap_or(());

            sqlx::query!(
                "UPDATE urls SET click_count = click_count + 1 WHERE short_code = $1",
                code
            )
            .execute(&state.db)
            .await?;
            tracing::info!(short_code = %code, original_url = %r.original_url, "redirecting");
            Ok(Redirect::to(&r.original_url).into_response())
        }
        None => Err(AppError::NotFound),
    }
}
