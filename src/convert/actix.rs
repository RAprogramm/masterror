//! Actix Web integration: `ResponseError` for [`AppError`] and helper JSON
//! payload.
//!
//! Enabled with the `actix` feature flag.
//!
//! ## What it does
//! - Implements `actix_web::ResponseError` for [`AppError`].
//!   - This lets you `return AppResult<_>` from Actix handlers.
//!   - On error, Actix automatically builds an `HttpResponse` with the right
//!     status code and JSON body (when the `serde_json` feature is enabled).
//! - Provides stable mapping from [`AppErrorKind`] to
//!   `actix_web::http::StatusCode`.
//! - Ensures that only safe, public-facing fields are returned to the client
//!   (`status`, `message`, `details?`).
//!
//! ## Wire payload
//!
//! When the `serde_json` feature is enabled, the body is [`ErrorResponse`]
//! with:
//! - `status`: numeric HTTP status (e.g. 404, 422, 500)
//! - `message`: explicit application message or a fallback from `AppErrorKind`
//! - `details`: currently `None`, but reserved for optional JSON/text payloads
//!
//! Without `serde_json`, Actix still returns a response with the correct status
//! but with an empty body.
//!
//! ## Example
//!
//! ```rust,ignore
//! use actix_web::{get, App, HttpServer};
//! use masterror::{AppError, AppErrorKind, AppResult};
//!
//! #[get("/forbidden")]
//! async fn forbidden() -> AppResult<&'static str> {
//!     Err(AppError::new(AppErrorKind::Forbidden, "no access"))
//! }
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| App::new().service(forbidden))
//!         .bind(("127.0.0.1", 8080))?
//!         .run()
//!         .await
//! }
//! ```
//!
//! The client will get a `403 Forbidden` response with a JSON body like:
//!
//! ```json
//! {"status":403,"message":"no access"}
//! ```
//!
//! ## Notes
//!
//! - Do not duplicate this `ResponseError` implementation elsewhere.
//! - Internal error sources (`std::error::Error` chain) are logged only; they
//!   are never leaked to the HTTP response.
//! - You typically want both `actix` and `serde_json` features enabled for
//!   proper JSON payloads.
//!
//! See also: Axum integration in [`convert::axum`].

#[cfg(feature = "actix")]
use actix_web::{
    HttpResponse, ResponseError,
    http::{
        StatusCode as ActixStatus,
        header::{RETRY_AFTER, WWW_AUTHENTICATE}
    }
};

#[cfg(feature = "actix")]
use crate::{AppError, ErrorResponse};

#[cfg(feature = "actix")]
impl ResponseError for AppError {
    /// Map to Actix `StatusCode` using the stable `AppErrorKind` → HTTP
    /// mapping.
    fn status_code(&self) -> ActixStatus {
        ActixStatus::from_u16(self.kind.http_status())
            .unwrap_or(ActixStatus::INTERNAL_SERVER_ERROR)
    }

    /// Produce JSON body with `ErrorResponse`. Does not leak sources.
    fn error_response(&self) -> HttpResponse {
        let body = ErrorResponse::from(self);
        let mut builder = HttpResponse::build(self.status_code());
        if let Some(retry) = body.retry {
            builder.insert_header((RETRY_AFTER, retry.after_seconds.to_string()));
        }
        if let Some(ref ch) = body.www_authenticate {
            builder.insert_header((WWW_AUTHENTICATE, ch.as_str()));
        }
        builder.json(body)
    }
}

#[cfg(all(test, feature = "actix"))]
mod actix_tests {
    use actix_web::{
        ResponseError,
        body::to_bytes,
        http::header::{RETRY_AFTER, WWW_AUTHENTICATE}
    };

    use crate::{AppCode, AppError, AppErrorKind, ErrorResponse};

    #[test]
    fn maps_status_consistently() {
        let e = AppError::new(AppErrorKind::Validation, "bad");
        assert_eq!(e.status_code().as_u16(), 422);
    }

    #[actix_web::test] // ← вот это
    async fn error_response_sets_body_and_headers() -> Result<(), Box<dyn std::error::Error>> {
        let err = AppError::unauthorized("no token")
            .with_retry_after_secs(7)
            .with_www_authenticate("Bearer");

        let resp = err.error_response();
        assert_eq!(resp.status().as_u16(), 401);

        let headers = resp.headers().clone();
        assert_eq!(
            headers.get(RETRY_AFTER).and_then(|v| v.to_str().ok()),
            Some("7")
        );
        assert_eq!(
            headers.get(WWW_AUTHENTICATE).and_then(|v| v.to_str().ok()),
            Some("Bearer")
        );

        let bytes = to_bytes(resp.into_body()).await?;
        let body: ErrorResponse = serde_json::from_slice(&bytes)?;
        assert_eq!(body.status, 401);
        assert!(matches!(body.code, AppCode::Unauthorized));
        assert_eq!(body.message, "no token");
        Ok(())
    }
}
