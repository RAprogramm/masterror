// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Actix integration: implements [`Responder`] for [`ProblemJson`] and
//! [`ErrorResponse`].
//!
//! Behavior:
//! - Serializes the response as RFC7807 `application/problem+json`.
//! - Adds `Retry-After` when retry advice is present.
//! - Adds `WWW-Authenticate` when an authentication challenge is provided.
//! - Redacts message and metadata when the error is marked private.

use actix_web::{
    HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    http::header::{CONTENT_TYPE, RETRY_AFTER, WWW_AUTHENTICATE}
};
use itoa::Buffer as IntegerBuffer;

use super::{ErrorResponse, ProblemJson};

pub(crate) fn respond_with_problem_json(mut problem: ProblemJson) -> HttpResponse {
    let http_status = problem.status_code();
    let status = actix_web::http::StatusCode::from_u16(http_status.as_u16())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    let retry_after = problem.retry_after;
    let www_authenticate = problem.www_authenticate.take();

    let mut response = HttpResponse::build(status).json(problem);

    response
        .headers_mut()
        .insert(CONTENT_TYPE, "application/problem+json".parse().unwrap());

    if let Some(retry) = retry_after {
        let mut buffer = IntegerBuffer::new();
        let retry_str = buffer.format(retry);
        if let Ok(hv) = retry_str.parse() {
            response.headers_mut().insert(RETRY_AFTER, hv);
        }
    }
    if let Some(challenge) = www_authenticate
        && let Ok(hv) = challenge.parse()
    {
        response.headers_mut().insert(WWW_AUTHENTICATE, hv);
    }

    response
}

impl Responder for ProblemJson {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        respond_with_problem_json(self)
    }
}

impl Responder for ErrorResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        respond_with_problem_json(ProblemJson::from_error_response(self))
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{
        Responder,
        http::header::{CONTENT_TYPE, RETRY_AFTER, WWW_AUTHENTICATE},
        test
    };

    use super::respond_with_problem_json;
    use crate::{AppCode, AppError, ErrorResponse, ProblemJson, response::core::RetryAdvice};

    #[actix_web::test]
    async fn respond_with_problem_json_sets_status_and_content_type() {
        let problem = ProblemJson::from_app_error(AppError::not_found("missing resource"));
        let response = respond_with_problem_json(problem);

        assert_eq!(response.status(), 404);
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(content_type, Some("application/problem+json"));
    }

    #[actix_web::test]
    async fn respond_with_problem_json_includes_retry_after_header() {
        let error = AppError::rate_limited("too many requests").with_retry_after_secs(60);
        let problem = ProblemJson::from_app_error(error);
        let response = respond_with_problem_json(problem);

        assert_eq!(response.status(), 429);
        let retry = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok());
        assert_eq!(retry, Some("60"));
    }

    #[actix_web::test]
    async fn respond_with_problem_json_includes_www_authenticate_header() {
        let error =
            AppError::unauthorized("invalid token").with_www_authenticate("Bearer realm=\"api\"");
        let problem = ProblemJson::from_app_error(error);
        let response = respond_with_problem_json(problem);

        assert_eq!(response.status(), 401);
        let auth = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(auth, Some("Bearer realm=\"api\""));
    }

    #[actix_web::test]
    async fn respond_with_problem_json_includes_both_retry_and_auth_headers() {
        let error = AppError::rate_limited("rate limit")
            .with_retry_after_secs(30)
            .with_www_authenticate("Bearer");
        let problem = ProblemJson::from_app_error(error);
        let response = respond_with_problem_json(problem);

        assert_eq!(response.status(), 429);
        let retry = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok());
        assert_eq!(retry, Some("30"));
        let auth = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(auth, Some("Bearer"));
    }

    #[actix_web::test]
    async fn problem_json_responder_returns_valid_response() {
        let req = test::TestRequest::default().to_http_request();
        let problem = ProblemJson::from_app_error(AppError::bad_request("invalid input"));
        let response = problem.respond_to(&req);

        assert_eq!(response.status(), 400);
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(content_type, Some("application/problem+json"));
    }

    #[actix_web::test]
    async fn error_response_responder_converts_and_responds() {
        let req = test::TestRequest::default().to_http_request();
        let error_response =
            ErrorResponse::new(503, AppCode::Service, "service down").expect("valid status");
        let response = error_response.respond_to(&req);

        assert_eq!(response.status(), 503);
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(content_type, Some("application/problem+json"));
    }

    #[actix_web::test]
    async fn error_response_responder_with_retry_header() {
        let req = test::TestRequest::default().to_http_request();
        let mut error_response =
            ErrorResponse::new(503, AppCode::Service, "service down").expect("valid status");
        error_response.retry = Some(RetryAdvice {
            after_seconds: 120
        });
        let response = error_response.respond_to(&req);

        assert_eq!(response.status(), 503);
        let retry = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok());
        assert_eq!(retry, Some("120"));
    }

    #[actix_web::test]
    async fn error_response_responder_with_www_authenticate() {
        let req = test::TestRequest::default().to_http_request();
        let mut error_response =
            ErrorResponse::new(401, AppCode::Unauthorized, "auth required").expect("valid status");
        error_response.www_authenticate = Some("Basic".to_owned());
        let response = error_response.respond_to(&req);

        assert_eq!(response.status(), 401);
        let auth = response
            .headers()
            .get(WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok());
        assert_eq!(auth, Some("Basic"));
    }
}
