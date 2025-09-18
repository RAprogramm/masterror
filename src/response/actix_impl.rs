//! Actix integration: implements [`Responder`] for [`ErrorResponse`].
//!
//! Behavior:
//! - Serializes the response as JSON with the given status.
//! - Adds `Retry-After` if [`ErrorResponse::retry`] is present.
//! - Adds `WWW-Authenticate` if [`ErrorResponse::www_authenticate`] is present.

use actix_web::{
    HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    http::header::{RETRY_AFTER, WWW_AUTHENTICATE}
};

use super::ErrorResponse;

impl Responder for ErrorResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let mut builder = HttpResponse::build(
            actix_web::http::StatusCode::from_u16(self.status)
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        );
        if let Some(retry) = self.retry {
            builder.insert_header((RETRY_AFTER, retry.after_seconds.to_string()));
        }
        if let Some(ref ch) = self.www_authenticate {
            // Pass &str, not &String, to satisfy TryIntoHeaderPair
            builder.insert_header((WWW_AUTHENTICATE, ch.as_str()));
        }
        builder.json(self)
    }
}
