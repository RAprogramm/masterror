//! Axum integration: implements [`IntoResponse`] for [`ProblemJson`] and
//! [`ErrorResponse`].
//!
//! Behavior:
//! - Serializes the response as `application/problem+json` with the given
//!   status.
//! - Adds `Retry-After` if retry advice is present.
//! - Adds `WWW-Authenticate` if an authentication challenge is present.
//! - Redacts the message and metadata when the error is marked as private.

use axum::{
    Json,
    http::{
        HeaderValue,
        header::{CONTENT_TYPE, RETRY_AFTER, WWW_AUTHENTICATE}
    },
    response::{IntoResponse, Response}
};
use itoa::Buffer as IntegerBuffer;

use super::{ErrorResponse, ProblemJson};

impl IntoResponse for ProblemJson {
    fn into_response(self) -> Response {
        let mut body = self;
        let status = body.status_code();
        let retry_after = body.retry_after;
        let www_authenticate = body.www_authenticate.take();
        let mut response = (status, Json(body)).into_response();

        response.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json")
        );

        if let Some(retry) = retry_after {
            let mut buffer = IntegerBuffer::new();
            let retry_str = buffer.format(retry);
            if let Ok(hv) = HeaderValue::from_str(retry_str) {
                response.headers_mut().insert(RETRY_AFTER, hv);
            }
        }
        if let Some(challenge) = www_authenticate
            && let Ok(hv) = HeaderValue::from_str(&challenge)
        {
            response.headers_mut().insert(WWW_AUTHENTICATE, hv);
        }

        response
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        ProblemJson::from_error_response(self).into_response()
    }
}
