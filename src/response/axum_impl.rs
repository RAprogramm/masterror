// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

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

#[cfg(test)]
mod tests {
    use axum::{
        http::{
            StatusCode,
            header::{CONTENT_TYPE, RETRY_AFTER, WWW_AUTHENTICATE}
        },
        response::IntoResponse
    };

    use crate::{AppCode, AppError, ErrorResponse, ProblemJson, response::core::RetryAdvice};

    #[tokio::test]
    async fn problem_json_into_response_sets_status_and_content_type() {
        let problem = ProblemJson::from_app_error(AppError::not_found("resource not found"));
        let response = problem.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(content_type, Some("application/problem+json"));
    }

    #[tokio::test]
    async fn problem_json_into_response_includes_retry_after_header() {
        let error = AppError::rate_limited("too many requests").with_retry_after_secs(120);
        let problem = ProblemJson::from_app_error(error);
        let response = problem.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        let retry = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok());
        assert_eq!(retry, Some("120"));
    }

    #[tokio::test]
    async fn problem_json_into_response_includes_www_authenticate_header() {
        let error = AppError::unauthorized("invalid credentials")
            .with_www_authenticate("Basic realm=\"api\"");
        let problem = ProblemJson::from_app_error(error);
        let response = problem.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let auth = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(auth, Some("Basic realm=\"api\""));
    }

    #[tokio::test]
    async fn problem_json_into_response_includes_both_retry_and_auth_headers() {
        let error = AppError::rate_limited("rate limited")
            .with_retry_after_secs(90)
            .with_www_authenticate("Bearer");
        let problem = ProblemJson::from_app_error(error);
        let response = problem.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        let retry = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok());
        assert_eq!(retry, Some("90"));
        let auth = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(auth, Some("Bearer"));
    }

    #[tokio::test]
    async fn error_response_into_response_converts_correctly() {
        let error_response =
            ErrorResponse::new(500, AppCode::Internal, "internal error").expect("valid status");
        let response = error_response.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(content_type, Some("application/problem+json"));
    }

    #[tokio::test]
    async fn error_response_into_response_with_retry_header() {
        let mut error_response = ErrorResponse::new(503, AppCode::Service, "service unavailable")
            .expect("valid status");
        error_response.retry = Some(RetryAdvice {
            after_seconds: 300
        });
        let response = error_response.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let retry = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok());
        assert_eq!(retry, Some("300"));
    }

    #[tokio::test]
    async fn error_response_into_response_with_www_authenticate() {
        let mut error_response =
            ErrorResponse::new(401, AppCode::Unauthorized, "auth required").expect("valid status");
        error_response.www_authenticate = Some("Digest realm=\"api\"".to_owned());
        let response = error_response.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let auth = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(auth, Some("Digest realm=\"api\""));
    }

    #[tokio::test]
    async fn problem_json_into_response_handles_various_status_codes() {
        let test_cases = vec![
            (AppError::bad_request("bad"), StatusCode::BAD_REQUEST),
            (AppError::conflict("conflict"), StatusCode::CONFLICT),
            (AppError::forbidden("forbidden"), StatusCode::FORBIDDEN),
            (
                AppError::internal("internal"),
                StatusCode::INTERNAL_SERVER_ERROR
            ),
        ];
        for (error, expected_status) in test_cases {
            let problem = ProblemJson::from_app_error(error);
            let response = problem.into_response();
            assert_eq!(response.status(), expected_status);
        }
    }
}
