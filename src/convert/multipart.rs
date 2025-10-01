// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Maps [`MultipartError`] into [`Error`] with
//! [`AppErrorKind::BadRequest`], preserving the original message.
//!
//! Intended for Axum multipart form parsing so that client mistakes are
//! surfaced as bad requests.

#![cfg(all(feature = "axum", feature = "multipart"))]

use axum::extract::multipart::MultipartError;

use crate::{AppErrorKind, Context, Error, field};

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
}
