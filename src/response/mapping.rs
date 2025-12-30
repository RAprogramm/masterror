// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Mapping from [`AppError`] to [`ErrorResponse`].
//!
//! This module provides [`From`] trait implementations and [`Display`]
//! formatting for converting application errors into HTTP-ready responses.
//!
//! # Conversions
//!
//! - [`From<AppError>`]: Consumes the error, transferring ownership of message
//!   and metadata
//! - [`From<&AppError>`]: Borrows the error, cloning message and metadata
//!
//! Both conversions respect the [`MessageEditPolicy`] to control message
//! visibility.
//!
//! # Display Format
//!
//! The [`Display`] implementation produces a concise log-safe format:
//! `"{status} {code:?}: {message}"`
//!
//! # Examples
//!
//! ```rust
//! use masterror::{AppError, ErrorResponse};
//!
//! let err = AppError::not_found("user not found");
//! let resp: ErrorResponse = err.into();
//!
//! assert_eq!(resp.status, 404);
//! assert_eq!(resp.message, "user not found");
//! ```
//!
//! [`MessageEditPolicy`]: crate::MessageEditPolicy

use alloc::string::String;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    mem::replace
};

use super::core::ErrorResponse;
use crate::{AppCode, AppError};

/// Format [`ErrorResponse`] for logging and debugging.
///
/// Produces a concise format: `"{status} {code:?}: {message}"`.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppCode, ErrorResponse};
///
/// let resp = ErrorResponse::new(404, AppCode::NotFound, "user not found").expect("status");
///
/// let formatted = resp.to_string();
/// assert!(formatted.contains("404"));
/// assert!(formatted.contains("NOT_FOUND"));
/// assert!(formatted.contains("user not found"));
/// ```
impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {:?}: {}", self.status, self.code, self.message)
    }
}

/// Convert owned [`AppError`] to [`ErrorResponse`].
///
/// Consumes the error, transferring ownership of message and metadata.
/// Respects [`MessageEditPolicy`] for message visibility.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppError, AppErrorKind, ErrorResponse};
///
/// let err = AppError::validation("invalid email");
/// let resp: ErrorResponse = err.into();
///
/// assert_eq!(resp.status, AppErrorKind::Validation.http_status());
/// assert_eq!(resp.message, "invalid email");
/// ```
///
/// [`MessageEditPolicy`]: crate::MessageEditPolicy
impl From<AppError> for ErrorResponse {
    fn from(mut err: AppError) -> Self {
        let kind = err.kind;
        let code = replace(&mut err.code, AppCode::from(kind));
        let retry = err.retry.take();
        let www_authenticate = err.www_authenticate.take();
        let policy = err.edit_policy;
        let status = kind.http_status();
        let message = match err.message.take() {
            Some(msg) if !matches!(policy, crate::MessageEditPolicy::Redact) => msg.into_owned(),
            _ => String::from(kind.label())
        };
        #[cfg(feature = "serde_json")]
        let details = if matches!(policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.take()
        };
        #[cfg(not(feature = "serde_json"))]
        let details = if matches!(policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.take()
        };
        Self {
            status,
            code,
            message,
            details,
            retry,
            www_authenticate
        }
    }
}

/// Convert borrowed [`AppError`] to [`ErrorResponse`].
///
/// Clones the error's message and metadata, leaving the original error intact.
/// Respects [`MessageEditPolicy`] for message visibility.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppError, ErrorResponse};
///
/// let err = AppError::conflict("resource exists");
/// let resp: ErrorResponse = (&err).into();
///
/// assert_eq!(resp.message, "resource exists");
/// // Original error remains intact
/// assert_eq!(err.message.as_deref(), Some("resource exists"));
/// ```
///
/// [`MessageEditPolicy`]: crate::MessageEditPolicy
impl From<&AppError> for ErrorResponse {
    fn from(err: &AppError) -> Self {
        let status = err.kind.http_status();
        let message = if matches!(err.edit_policy, crate::MessageEditPolicy::Redact) {
            String::from(err.kind.label())
        } else {
            err.render_message().into_owned()
        };
        #[cfg(feature = "serde_json")]
        let details = if matches!(err.edit_policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.clone()
        };
        #[cfg(not(feature = "serde_json"))]
        let details = if matches!(err.edit_policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.clone()
        };
        Self {
            status,
            code: err.code.clone(),
            message,
            details,
            retry: err.retry,
            www_authenticate: err.www_authenticate.clone()
        }
    }
}
