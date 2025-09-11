//! Maps [`MultipartError`] into [`AppError`] with
//! [`AppErrorKind::BadRequest`], preserving the original message.
//!
//! Intended for Axum multipart form parsing so that client mistakes are
//! surfaced as bad requests.

#![cfg(all(feature = "axum", feature = "multipart"))]

use axum::extract::multipart::MultipartError;

use crate::{AppError, AppErrorKind};

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::with(AppErrorKind::BadRequest, format!("Multipart error: {err}"))
    }
}

#[cfg(all(test, feature = "axum", feature = "multipart"))]
mod tests {
    use axum::{
        body::Body,
        extract::{FromRequest, multipart::Multipart},
        http::Request
    };

    use crate::{AppError, AppErrorKind};

    #[tokio::test]
    async fn multipart_error_maps_to_bad_request() {
        let boundary = "XBOUNDARY";
        let request = Request::builder()
            .header(
                "content-type",
                format!("multipart/form-data; boundary={boundary}")
            )
            .body(Body::from("not-a-multipart-body"))
            .expect("request");

        let mut multipart = Multipart::from_request(request, &())
            .await
            .expect("extractor");

        let err = multipart.next_field().await.expect_err("error");
        let expected = format!("Multipart error: {err}");
        let app_err: AppError = err.into();

        assert_eq!(app_err.kind, AppErrorKind::BadRequest);
        assert_eq!(app_err.message.as_deref(), Some(expected.as_str()));
    }
}
