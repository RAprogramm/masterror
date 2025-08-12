//! Conversion from [`tokio::time::error::Elapsed`] into [`AppError`].
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
//! use masterror::{AppError, AppErrorKind};
//! use tokio::time::{sleep, timeout, Duration};
//!
//! #[tokio::main]
//! async fn main() {
//!     let fut = sleep(Duration::from_secs(2));
//!     let res = timeout(Duration::from_millis(10), fut).await;
//!
//!     let err = res.unwrap_err(); // tokio::time::error::Elapsed
//!     let app_err: AppError = err.into();
//!
//!     assert!(matches!(app_err.kind, AppErrorKind::Timeout));
//! }
//! ```

#[cfg(feature = "tokio")]
use tokio::time::error::Elapsed;

#[cfg(feature = "tokio")]
use crate::AppError;

/// Map a [`tokio::time::error::Elapsed`] into an [`AppError`] with kind
/// `Timeout`.
///
/// Message is fixed to avoid leaking timing specifics to the client.
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl From<Elapsed> for AppError {
    fn from(_: Elapsed) -> Self {
        AppError::timeout("Operation timed out")
    }
}
