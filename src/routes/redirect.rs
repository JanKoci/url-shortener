use crate::errors::AppError;
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let row = sqlx::query!("SELECT original_url FROM urls WHERE short_code = $1", code)
        .fetch_optional(&state.db)
        .await?;
    match row {
        Some(r) => {
            sqlx::query!(
                "UPDATE urls SET click_count = click_count + 1 WHERE short_code = $1",
                code
            )
            .execute(&state.db)
            .await?;
            Ok(Redirect::to(&r.original_url).into_response())
        }
        None => Err(AppError::NotFound),
    }
}
