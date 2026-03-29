use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

use crate::{errors::AppError, utils::is_expired, AppState};

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
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
