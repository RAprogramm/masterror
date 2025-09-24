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

use super::{ErrorResponse, ProblemJson};

pub(crate) fn respond_with_problem_json(mut problem: ProblemJson) -> HttpResponse {
    let http_status = problem.status_code();
    let status = actix_web::http::StatusCode::from_u16(http_status.as_u16())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    let retry_after = problem.retry_after;
    let www_authenticate = problem.www_authenticate.take();

    let mut builder = HttpResponse::build(status);
    builder.insert_header((CONTENT_TYPE, "application/problem+json"));

    if let Some(retry) = retry_after {
        builder.insert_header((RETRY_AFTER, retry.to_string()));
    }
    if let Some(challenge) = www_authenticate {
        builder.insert_header((WWW_AUTHENTICATE, challenge));
    }

    builder.json(problem)
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
