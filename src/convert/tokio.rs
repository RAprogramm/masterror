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
//! ```rust,ignore
//! use masterror::{AppErrorKind, Error};
//! use tokio::time::{sleep, timeout, Duration};
//!
//! #[tokio::main]
//! async fn main() {
//!     let fut = sleep(Duration::from_secs(2));
//!     let res = timeout(Duration::from_millis(10), fut).await;
//!
//!     let err = res.unwrap_err(); // tokio::time::error::Elapsed
//!     let app_err: Error = err.into();
//!
//!     assert!(matches!(app_err.kind, AppErrorKind::Timeout));
//! }
//! ```

#[cfg(feature = "tokio")]
use tokio::time::error::Elapsed;

#[cfg(feature = "tokio")]
use crate::{
    AppErrorKind,
    app_error::{Context, Error, field}
};

/// Map a [`tokio::time::error::Elapsed`] into an [`AppError`] with kind
/// `Timeout`.
///
/// Message is fixed to avoid leaking timing specifics to the client.
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
}
