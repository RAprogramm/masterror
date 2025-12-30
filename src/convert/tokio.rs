// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion from [`tokio::time::error::Elapsed`] into [`Error`].
//!
//! Enabled with the `tokio` feature flag.
//!
//! ## Mapping
//!
//! All elapsed-time errors are mapped to `AppErrorKind::Timeout` with a fixed,
//! public-facing message `"Operation timed out"`.
//!
//! ## Rationale
//!
//! `tokio::time::error::Elapsed` occurs when a future wrapped in
//! [`tokio::time::timeout`] does not complete within the given duration.
//! This is an infrastructure/latency issue rather than a domain error,
//! so the mapping uses a `Timeout` kind that clients can handle specifically.
//!
//! ## Example
//!
//! ```rust
//! use masterror::{AppErrorKind, Error};
//! use tokio::time::{Duration, sleep, timeout};
//!
//! #[tokio::main]
//! async fn main() {
//!     let fut = sleep(Duration::from_secs(2));
//!     let res = timeout(Duration::from_millis(10), fut).await;
//!
//!     let err = res.unwrap_err();
//!     let app_err: Error = err.into();
//!
//!     assert!(matches!(app_err.kind, AppErrorKind::Timeout));
//! }
//! ```

#[cfg(feature = "tokio")]
use tokio::time::error::Elapsed;

#[cfg(feature = "tokio")]
use crate::{AppErrorKind, Context, Error, field};

/// Map a [`tokio::time::error::Elapsed`] into an [`crate::AppError`] with kind
/// `Timeout`.
///
/// Message is fixed to avoid leaking timing specifics to the client.
///
/// # Example
///
/// ```rust
/// use masterror::Error;
/// use tokio::time::{Duration, sleep, timeout};
///
/// #[tokio::main]
/// async fn main() {
///     let fut = sleep(Duration::from_millis(100));
///     let result = timeout(Duration::from_millis(1), fut).await;
///
///     let elapsed_err = result.unwrap_err();
///     let app_err: Error = elapsed_err.into();
///
///     assert_eq!(app_err.kind, masterror::AppErrorKind::Timeout);
/// }
/// ```
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl From<Elapsed> for Error {
    fn from(err: Elapsed) -> Self {
        Context::new(AppErrorKind::Timeout)
            .with(field::str("timeout.source", "tokio::time::timeout"))
            .into_error(err)
    }
}

#[cfg(all(test, feature = "tokio"))]
mod tests {
    use std::error::Error as StdError;

    use tokio::time::{Duration, sleep, timeout};

    use super::*;
    use crate::{AppErrorKind, FieldValue};

    #[tokio::test]
    async fn elapsed_maps_to_timeout() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::Timeout));
        assert_eq!(
            app_err.metadata().get("timeout.source"),
            Some(&FieldValue::Str("tokio::time::timeout".into()))
        );
    }

    #[tokio::test]
    async fn elapsed_conversion_preserves_source() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        assert!(app_err.source().is_some());
    }

    #[tokio::test]
    async fn elapsed_error_display() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        let display = format!("{}", app_err);
        assert!(!display.is_empty());
    }

    #[tokio::test]
    async fn elapsed_error_debug() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        let debug = format!("{:?}", app_err);
        assert!(debug.contains("Timeout"));
    }

    #[tokio::test]
    async fn elapsed_metadata_contains_source_field() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("timeout.source"),
            Some(&FieldValue::Str("tokio::time::timeout".into()))
        );
    }

    #[tokio::test]
    async fn multiple_elapsed_errors_convert_consistently() {
        let fut1 = sleep(Duration::from_millis(20));
        let err1 = timeout(Duration::from_millis(1), fut1)
            .await
            .expect_err("expect timeout");
        let app_err1: Error = err1.into();
        let fut2 = sleep(Duration::from_millis(30));
        let err2 = timeout(Duration::from_millis(1), fut2)
            .await
            .expect_err("expect timeout");
        let app_err2: Error = err2.into();
        assert!(matches!(app_err1.kind, AppErrorKind::Timeout));
        assert!(matches!(app_err2.kind, AppErrorKind::Timeout));
        assert_eq!(
            app_err1.metadata().get("timeout.source"),
            app_err2.metadata().get("timeout.source")
        );
    }

    #[tokio::test]
    async fn elapsed_kind_is_exactly_timeout() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::Timeout);
    }

    #[tokio::test]
    async fn elapsed_source_is_elapsed_type() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        let source = app_err.source().expect("source should exist");
        assert!(source.is::<Elapsed>());
    }

    #[tokio::test]
    async fn very_short_timeout_converts() {
        let fut = sleep(Duration::from_millis(100));
        let err = timeout(Duration::from_nanos(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::Timeout));
    }

    #[tokio::test]
    async fn elapsed_error_implements_std_error() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        let _: &dyn StdError = &app_err;
    }

    #[tokio::test]
    async fn elapsed_conversion_from_reference() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Timeout));
    }

    #[tokio::test]
    async fn elapsed_metadata_is_not_empty() {
        let fut = sleep(Duration::from_millis(20));
        let err = timeout(Duration::from_millis(1), fut)
            .await
            .expect_err("expect timeout");
        let app_err: Error = err.into();
        assert!(!app_err.metadata().is_empty());
    }
}
