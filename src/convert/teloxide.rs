// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion from [`teloxide_core::RequestError`] into [`Error`].
//!
//! Enabled with the `teloxide` feature flag.
//!
//! ## Mapping
//!
//! - [`RequestError::Api`] → `AppErrorKind::ExternalApi` (invalid token →
//!   `AppErrorKind::Unauthorized`)
//! - [`RequestError::MigrateToChatId`] → `AppErrorKind::ExternalApi`
//! - [`RequestError::RetryAfter`] → `AppErrorKind::RateLimited`
//! - [`RequestError::Network`] → `AppErrorKind::Network`
//! - [`RequestError::InvalidJson`] → `AppErrorKind::Deserialization`
//! - [`RequestError::Io`] → `AppErrorKind::Internal`
//!
//! The original error string is preserved in the `message` for observability.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppErrorKind, Error};
//! use teloxide_core::{errors::ApiError, RequestError, types::Seconds};
//! use std::{io, sync::Arc};
//!
//! fn map(err: RequestError) -> Error { err.into() }
//!
//! let err = RequestError::RetryAfter(Seconds::from_seconds(1));
//! let app_err = map(err);
//! assert!(matches!(app_err.kind, AppErrorKind::RateLimited));
//! ```
#[cfg(feature = "teloxide")]
use teloxide_core::{RequestError, errors::ApiError};

#[cfg(feature = "teloxide")]
use crate::{AppErrorKind, Context, Error, FieldRedaction, field};

#[cfg(feature = "teloxide")]
#[cfg_attr(docsrs, doc(cfg(feature = "teloxide")))]
impl From<RequestError> for Error {
    fn from(err: RequestError) -> Self {
        let (context, retry_after) = build_teloxide_context(&err);
        let mut error = context.into_error(err);
        if let Some(secs) = retry_after {
            error = error.with_retry_after_secs(secs);
        }
        error
    }
}

#[cfg(feature = "teloxide")]
fn build_teloxide_context(err: &RequestError) -> (Context, Option<u64>) {
    match err {
        RequestError::Api(api) => {
            let mut context = Context::new(AppErrorKind::ExternalApi)
                .with(field::str("telegram.reason", "api"))
                .with(field::str("telegram.api_error", api.to_string()))
                .with(field::str(
                    "telegram.api_error_variant",
                    format!("{:?}", api)
                ));

            if matches!(api, ApiError::InvalidToken) {
                context = context.category(AppErrorKind::Unauthorized);
            }

            (context, None)
        }
        RequestError::MigrateToChatId(id) => (
            Context::new(AppErrorKind::ExternalApi)
                .with(field::str("telegram.reason", "migrate_to_chat"))
                .with(field::i64("telegram.chat_id", id.0)),
            None
        ),
        RequestError::RetryAfter(secs) => {
            let seconds = u64::from(secs.seconds());
            (
                Context::new(AppErrorKind::RateLimited)
                    .with(field::str("telegram.reason", "retry_after"))
                    .with(field::u64("telegram.retry_after_secs", seconds)),
                Some(seconds)
            )
        }
        RequestError::Network(e) => (
            Context::new(AppErrorKind::Network)
                .with(field::str("telegram.reason", "network"))
                .with(field::str("telegram.detail", e.to_string()))
                .redact_field("telegram.detail", FieldRedaction::Hash),
            None
        ),
        RequestError::InvalidJson {
            source,
            raw
        } => (
            Context::new(AppErrorKind::Deserialization)
                .with(field::str("telegram.reason", "invalid_json"))
                .with(field::str("telegram.detail", source.to_string()))
                .with(field::u64("telegram.payload_len", raw.len() as u64)),
            None
        ),
        RequestError::Io(e) => (
            Context::new(AppErrorKind::Internal)
                .with(field::str("telegram.reason", "io"))
                .with(field::str("io.kind", format!("{:?}", e.kind()))),
            None
        )
    }
}

#[cfg(all(test, feature = "teloxide"))]
mod tests {
    #[cfg(feature = "reqwest")]
    use std::time::Duration;
    use std::{io, sync::Arc};

    use teloxide_core::{errors::ApiError, types::Seconds};
    #[cfg(feature = "reqwest")]
    use tokio::runtime::Builder;

    use super::*;
    #[cfg(feature = "reqwest")]
    use crate::FieldRedaction;
    use crate::{AppCode, AppErrorKind, FieldValue};

    #[test]
    fn api_maps_to_external_api() {
        let err = RequestError::Api(ApiError::BotBlocked);
        let app_err: Error = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::ExternalApi));
        assert_eq!(
            app_err.metadata().get("telegram.api_error"),
            Some(&FieldValue::Str(ApiError::BotBlocked.to_string().into()))
        );
    }

    #[test]
    fn retry_after_maps_to_rate_limited() {
        let err = RequestError::RetryAfter(Seconds::from_seconds(5));
        let app_err: Error = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::RateLimited));
        assert_eq!(app_err.retry.map(|r| r.after_seconds), Some(5));
    }

    #[test]
    fn io_maps_to_internal() {
        let io_err = Arc::new(io::Error::other("disk"));
        let err = RequestError::Io(io_err);
        let app_err: Error = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::Internal));
        assert_eq!(
            app_err.metadata().get("telegram.reason"),
            Some(&FieldValue::Str("io".into()))
        );
    }

    #[test]
    fn invalid_token_maps_to_unauthorized() {
        let err = RequestError::Api(ApiError::InvalidToken);
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::Unauthorized);
        assert_eq!(app_err.code, AppCode::Unauthorized);
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("telegram.api_error_variant"),
            Some(&FieldValue::Str("InvalidToken".into()))
        );
    }

    #[cfg(feature = "reqwest")]
    #[test]
    fn network_detail_is_hashed() {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        let reqwest_err = runtime.block_on(async {
            reqwest::Client::builder()
                .timeout(Duration::from_millis(10))
                .build()
                .expect("client")
                .get("http://127.0.0.1:65535")
                .send()
                .await
                .expect_err("expected failure")
        });
        let err = RequestError::Network(Arc::new(reqwest_err));
        let app_err: Error = err.into();
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.redaction("telegram.detail"),
            Some(FieldRedaction::Hash)
        );
    }
}
