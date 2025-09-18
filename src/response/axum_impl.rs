//! Axum integration: implements [`IntoResponse`] for [`ErrorResponse`].
//!
//! Behavior:
//! - Serializes the response as JSON with the given status.
//! - Adds `Retry-After` if [`ErrorResponse::retry`] is present.
//! - Adds `WWW-Authenticate` if [`ErrorResponse::www_authenticate`] is present.

use axum::{
    Json,
    http::{
        HeaderValue,
        header::{RETRY_AFTER, WWW_AUTHENTICATE}
    },
    response::{IntoResponse, Response}
};

use super::ErrorResponse;
use crate::AppError;

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = self.status_code();

        // Serialize JSON body first (borrow self for payload).
        let mut response = (status, Json(&self)).into_response();

        if let Some(retry) = self.retry
            && let Ok(hv) = HeaderValue::from_str(&retry.after_seconds.to_string())
        {
            response.headers_mut().insert(RETRY_AFTER, hv);
        }
        if let Some(ch) = &self.www_authenticate
            && let Ok(hv) = HeaderValue::from_str(ch)
        {
            response.headers_mut().insert(WWW_AUTHENTICATE, hv);
        }

        response
    }
}

/// Convert `AppError` into the stable wire model and reuse its `IntoResponse`.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Use the canonical mapping defined in `From<&AppError> for ErrorResponse`
        let wire: ErrorResponse = (&self).into();
        wire.into_response()
    }
}
