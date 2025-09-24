//! Maps [`MultipartError`] into [`Error`] with
//! [`AppErrorKind::BadRequest`], preserving the original message.
//!
//! Intended for Axum multipart form parsing so that client mistakes are
//! surfaced as bad requests.

#![cfg(all(feature = "axum", feature = "multipart"))]

use axum::extract::multipart::MultipartError;

use crate::{
    AppErrorKind,
    app_error::{Context, Error, field}
};

impl From<MultipartError> for Error {
    fn from(err: MultipartError) -> Self {
        Context::new(AppErrorKind::BadRequest)
            .with(field::str("multipart.reason", err.to_string()))
            .into_error(err)
    }
}

#[cfg(all(test, feature = "axum", feature = "multipart"))]
mod tests {
    use axum::{
        body::Body,
        extract::{FromRequest, multipart::Multipart},
        http::Request
    };

    use crate::{AppErrorKind, FieldValue};

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
        let app_err: Error = err.into();

        assert_eq!(app_err.kind, AppErrorKind::BadRequest);
        assert_eq!(
            app_err.metadata().get("multipart.reason"),
            Some(&FieldValue::Str(err.to_string().into()))
        );
    }
}
