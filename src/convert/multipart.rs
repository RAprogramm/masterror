#![cfg(all(feature = "axum", feature = "multipart"))]

use axum::extract::multipart::MultipartError;

use crate::AppError;

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::bad_request(format!("Multipart error: {err}"))
    }
}
