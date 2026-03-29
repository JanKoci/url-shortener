use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    NotFound,
    BadRequest(String),
    InternalServerError,
    Gone,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            AppError::Gone => (StatusCode::GONE, "URL entry is expired".to_string()),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_error: sqlx::Error) -> Self {
        AppError::InternalServerError
    }
}
