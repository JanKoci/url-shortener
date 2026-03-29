use axum::{
    extract::{Path, State},
    http::{header, HeaderMap},
};
use image::Luma;
use qrcode::{types::QrError, QrCode};

use crate::{errors::AppError, utils::is_expired, AppState};

pub async fn qr_handler(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<(HeaderMap, Vec<u8>), AppError> {
    let row = sqlx::query!(
        "SELECT original_url, expires_at FROM urls WHERE short_code = $1",
        code
    )
    .fetch_optional(&state.db)
    .await?;
    match row {
        Some(r) => {
            if is_expired(r.expires_at) {
                return Err(AppError::Gone);
            }
            let qr = QrCode::new(r.original_url.as_bytes())
                .map_err(|_: QrError| AppError::InternalServerError)?;
            let image = qr.render::<Luma<u8>>().build();

            let mut png_bytes: Vec<u8> = Vec::new();
            image
                .write_to(
                    &mut std::io::Cursor::new(&mut png_bytes),
                    image::ImageFormat::Png,
                )
                .map_err(|_| AppError::InternalServerError)?;

            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
            Ok((headers, png_bytes))
        }
        None => Err(AppError::NotFound),
    }
}
