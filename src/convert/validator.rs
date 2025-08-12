//! Conversion from [`validator::ValidationErrors`] into [`AppError`].
//!
//! Enabled with the `validator` feature flag.
//!
//! ## Mapping
//!
//! All validation failures are mapped to `AppErrorKind::Validation` with
//! the stringified [`ValidationErrors`] content in `message`.
//!
//! ## Rationale
//!
//! [`validator::ValidationErrors`] provides structured error details, but
//! serializing them directly into the public API payload is not always desired.
//! Here we convert them to a human-readable string for logs and simple clients.
//! If you need to expose structured validation errors in JSON, extend your
//! `AppError` type to carry `serde_json::Value` and adjust this mapping
//! accordingly.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppError, AppErrorKind};
//! use validator::{Validate, ValidationError};
//!
//! #[derive(Validate)]
//! struct Payload {
//!     #[validate(length(min = 5))]
//!     name: String,
//! }
//!
//! fn validate_payload(p: Payload) -> Result<(), AppError> {
//!     p.validate()?;
//!     Ok(())
//! }
//!
//! let bad = Payload { name: "abc".into() };
//! let err = validate_payload(bad).unwrap_err();
//! assert!(matches!(err.kind, AppErrorKind::Validation));
//! ```

#[cfg(feature = "validator")]
use validator::ValidationErrors;

#[cfg(feature = "validator")]
use crate::AppError;

/// Map [`validator::ValidationErrors`] into an [`AppError`] with kind
/// `Validation`.
///
/// By default, the error is converted to a string for human-readable logs.
/// Consider extending `AppError` if you want to expose structured details.
#[cfg(feature = "validator")]
#[cfg_attr(docsrs, doc(cfg(feature = "validator")))]
impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::validation(err.to_string())
    }
}
