// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Maps [`MultipartError`] into [`Error`] with
//! [`AppErrorKind::BadRequest`], preserving the original message.
//!
//! Intended for Axum multipart form parsing so that client mistakes are
//! surfaced as bad requests.
//!
//! ## Example
//!
//! ```rust
//! use axum::{
//!     body::Body,
//!     extract::{FromRequest, multipart::Multipart},
//!     http::Request
//! };
//! use masterror::{AppErrorKind, Error};
//!
//! #[tokio::main]
//! async fn main() {
//!     let boundary = "XBOUNDARY";
//!     let request = Request::builder()
//!         .header(
//!             "content-type",
//!             format!("multipart/form-data; boundary={boundary}")
//!         )
//!         .body(Body::from("invalid"))
//!         .expect("request");
//!
//!     let mut multipart = Multipart::from_request(request, &())
//!         .await
//!         .expect("extractor");
//!
//!     let err = multipart.next_field().await.expect_err("error");
//!     let app_err: Error = err.into();
//!
//!     assert_eq!(app_err.kind, AppErrorKind::BadRequest);
//! }
//! ```

#![cfg(all(feature = "axum", feature = "multipart"))]

use axum::extract::multipart::MultipartError;

use crate::{AppErrorKind, Context, Error, field};

/// Convert a [`MultipartError`] into an [`struct@crate::Error`] with
/// [`AppErrorKind::BadRequest`].
///
/// All multipart parsing errors are treated as client errors since they
/// indicate malformed input.
///
/// # Example
///
/// ```rust
/// use axum::{
///     body::Body,
///     extract::{FromRequest, multipart::Multipart},
///     http::Request
/// };
/// use masterror::Error;
///
/// #[tokio::main]
/// async fn main() {
///     let request = Request::builder()
///         .header("content-type", "multipart/form-data; boundary=X")
///         .body(Body::from("bad"))
///         .expect("request");
///
///     let mut multipart = Multipart::from_request(request, &())
///         .await
///         .expect("extractor");
///
///     if let Err(err) = multipart.next_field().await {
///         let app_err: Error = err.into();
///         assert_eq!(app_err.kind, masterror::AppErrorKind::BadRequest);
///     }
/// }
/// ```
impl From<MultipartError> for Error {
    fn from(err: MultipartError) -> Self {
        let status = err.status();
        let body_text = err.body_text();
        let mut context = Context::new(AppErrorKind::BadRequest)
            .with(field::str("multipart.reason", body_text))
            .with(field::u64("http.status", u64::from(status.as_u16())))
            .with(field::bool(
                "http.is_client_error",
                status.is_client_error()
            ));
        if let Some(reason) = status.canonical_reason() {
            context = context.with(field::str("http.status_reason", reason));
        }
        context.into_error(err)
    }
}

#[cfg(all(test, feature = "axum", feature = "multipart"))]
mod tests {
    use axum::{
        body::Body,
        extract::{FromRequest, multipart::Multipart},
        http::Request
    };

    use crate::{AppErrorKind, Error, FieldValue};

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
        let status = err.status();
        let body_text = err.body_text();
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::BadRequest);
        assert_eq!(
            app_err.metadata().get("multipart.reason"),
            Some(&FieldValue::Str(body_text.into()))
        );
        assert_eq!(
            app_err.metadata().get("http.status"),
            Some(&FieldValue::U64(u64::from(status.as_u16())))
        );
        assert_eq!(
            app_err.metadata().get("http.status_reason"),
            status
                .canonical_reason()
                .map(|reason| FieldValue::Str(reason.into()))
                .as_ref()
        );
        assert_eq!(
            app_err.metadata().get("http.is_client_error"),
            Some(&FieldValue::Bool(status.is_client_error()))
        );
    }

    #[tokio::test]
    async fn malformed_boundary_maps_to_bad_request() {
        let request = Request::builder()
            .header("content-type", "multipart/form-data; boundary=X")
            .body(Body::from("invalid-multipart-data"))
            .expect("request");
        let mut multipart = Multipart::from_request(request, &())
            .await
            .expect("extractor");
        let err = multipart.next_field().await.expect_err("error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::BadRequest);
        assert!(app_err.metadata().get("multipart.reason").is_some());
    }

    #[tokio::test]
    async fn empty_body_multipart_error() {
        let request = Request::builder()
            .header("content-type", "multipart/form-data; boundary=BOUND")
            .body(Body::empty())
            .expect("request");
        let mut multipart = Multipart::from_request(request, &())
            .await
            .expect("extractor");
        let err = multipart.next_field().await.expect_err("error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::BadRequest);
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("http.is_client_error"),
            Some(&FieldValue::Bool(true))
        );
    }

    #[tokio::test]
    async fn metadata_contains_http_status() {
        let request = Request::builder()
            .header("content-type", "multipart/form-data; boundary=TEST")
            .body(Body::from("garbage"))
            .expect("request");
        let mut multipart = Multipart::from_request(request, &())
            .await
            .expect("extractor");
        let err = multipart.next_field().await.expect_err("error");
        let app_err: Error = err.into();
        let metadata = app_err.metadata();
        assert!(metadata.get("http.status").is_some());
        assert!(metadata.get("http.status_reason").is_some());
    }

    #[tokio::test]
    async fn error_preserves_original_message() {
        let request = Request::builder()
            .header("content-type", "multipart/form-data; boundary=B")
            .body(Body::from("bad-data"))
            .expect("request");
        let mut multipart = Multipart::from_request(request, &())
            .await
            .expect("extractor");
        let err = multipart.next_field().await.expect_err("error");
        let original_message = err.body_text();
        let app_err: Error = err.into();
        assert_eq!(
            app_err.metadata().get("multipart.reason"),
            Some(&FieldValue::Str(original_message.into()))
        );
    }

    #[tokio::test]
    async fn status_is_client_error() {
        let request = Request::builder()
            .header("content-type", "multipart/form-data; boundary=XYZ")
            .body(Body::from("invalid"))
            .expect("request");
        let mut multipart = Multipart::from_request(request, &())
            .await
            .expect("extractor");
        let err = multipart.next_field().await.expect_err("error");
        let status = err.status();
        let app_err: Error = err.into();
        assert!(status.is_client_error());
        assert_eq!(app_err.kind, AppErrorKind::BadRequest);
    }

    #[tokio::test]
    async fn conversion_creates_valid_error() {
        let request = Request::builder()
            .header("content-type", "multipart/form-data; boundary=ABC")
            .body(Body::from("malformed"))
            .expect("request");
        let mut multipart = Multipart::from_request(request, &())
            .await
            .expect("extractor");
        let err = multipart.next_field().await.expect_err("error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::BadRequest);
        assert!(app_err.source.is_some());
    }

    #[tokio::test]
    async fn multiple_errors_convert_consistently() {
        let request1 = Request::builder()
            .header("content-type", "multipart/form-data; boundary=A")
            .body(Body::from("bad1"))
            .expect("request");
        let mut multipart1 = Multipart::from_request(request1, &())
            .await
            .expect("extractor");
        let err1 = multipart1.next_field().await.expect_err("error");
        let app_err1: Error = err1.into();
        let request2 = Request::builder()
            .header("content-type", "multipart/form-data; boundary=B")
            .body(Body::from("bad2"))
            .expect("request");
        let mut multipart2 = Multipart::from_request(request2, &())
            .await
            .expect("extractor");
        let err2 = multipart2.next_field().await.expect_err("error");
        let app_err2: Error = err2.into();
        assert_eq!(app_err1.kind, AppErrorKind::BadRequest);
        assert_eq!(app_err2.kind, AppErrorKind::BadRequest);
    }
}
