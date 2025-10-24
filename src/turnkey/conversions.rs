// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversions from Turnkey domain errors to [`AppError`].
//!
//! This module provides [`From`] trait implementations for converting
//! Turnkey-specific errors into the canonical application error types.
//!
//! # Conversions
//!
//! - [`TurnkeyErrorKind`] → [`AppErrorKind`]: Uses [`map_turnkey_kind`]
//! - [`TurnkeyError`] → [`AppError`]: Preserves message and maps kind
//!
//! # Examples
//!
//! ```rust
//! use masterror::{
//!     AppError, AppErrorKind,
//!     turnkey::{TurnkeyError, TurnkeyErrorKind}
//! };
//!
//! let turnkey_err = TurnkeyError::new(TurnkeyErrorKind::Timeout, "operation timed out");
//! let app_err: AppError = turnkey_err.into();
//!
//! assert_eq!(app_err.kind, AppErrorKind::Timeout);
//! assert_eq!(app_err.message.as_deref(), Some("operation timed out"));
//! ```

use super::domain::{TurnkeyError, TurnkeyErrorKind, map_turnkey_kind};
use crate::{AppError, AppErrorKind};

/// Convert [`TurnkeyErrorKind`] to [`AppErrorKind`].
///
/// Uses [`map_turnkey_kind`] to perform the conversion.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppErrorKind, turnkey::TurnkeyErrorKind};
///
/// let kind: AppErrorKind = TurnkeyErrorKind::Timeout.into();
/// assert_eq!(kind, AppErrorKind::Timeout);
///
/// let kind: AppErrorKind = TurnkeyErrorKind::Auth.into();
/// assert_eq!(kind, AppErrorKind::Unauthorized);
/// ```
impl From<TurnkeyErrorKind> for AppErrorKind {
    #[inline]
    fn from(k: TurnkeyErrorKind) -> Self {
        map_turnkey_kind(k)
    }
}

/// Convert [`TurnkeyError`] to [`AppError`].
///
/// Preserves the error message and maps the kind using explicit constructors
/// to maintain consistent transport-layer mapping.
///
/// # Examples
///
/// ```rust
/// use masterror::{
///     AppError, AppErrorKind,
///     turnkey::{TurnkeyError, TurnkeyErrorKind}
/// };
///
/// let turnkey_err = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "quota exceeded");
/// let app_err: AppError = turnkey_err.into();
///
/// assert_eq!(app_err.kind, AppErrorKind::RateLimited);
/// assert_eq!(app_err.message.as_deref(), Some("quota exceeded"));
/// ```
impl From<TurnkeyError> for AppError {
    #[inline]
    fn from(e: TurnkeyError) -> Self {
        match e.kind {
            TurnkeyErrorKind::UniqueLabel => AppError::conflict(e.msg),
            TurnkeyErrorKind::RateLimited => AppError::rate_limited(e.msg),
            TurnkeyErrorKind::Timeout => AppError::timeout(e.msg),
            TurnkeyErrorKind::Auth => AppError::unauthorized(e.msg),
            TurnkeyErrorKind::Network => AppError::network(e.msg),
            TurnkeyErrorKind::Service => AppError::turnkey(e.msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turnkey_error_kind_to_app_error_kind_unique_label() {
        let kind: AppErrorKind = TurnkeyErrorKind::UniqueLabel.into();
        assert_eq!(kind, AppErrorKind::Conflict);
    }

    #[test]
    fn turnkey_error_kind_to_app_error_kind_rate_limited() {
        let kind: AppErrorKind = TurnkeyErrorKind::RateLimited.into();
        assert_eq!(kind, AppErrorKind::RateLimited);
    }

    #[test]
    fn turnkey_error_kind_to_app_error_kind_timeout() {
        let kind: AppErrorKind = TurnkeyErrorKind::Timeout.into();
        assert_eq!(kind, AppErrorKind::Timeout);
    }

    #[test]
    fn turnkey_error_kind_to_app_error_kind_auth() {
        let kind: AppErrorKind = TurnkeyErrorKind::Auth.into();
        assert_eq!(kind, AppErrorKind::Unauthorized);
    }

    #[test]
    fn turnkey_error_kind_to_app_error_kind_network() {
        let kind: AppErrorKind = TurnkeyErrorKind::Network.into();
        assert_eq!(kind, AppErrorKind::Network);
    }

    #[test]
    fn turnkey_error_kind_to_app_error_kind_service() {
        let kind: AppErrorKind = TurnkeyErrorKind::Service.into();
        assert_eq!(kind, AppErrorKind::Turnkey);
    }

    #[test]
    fn turnkey_error_to_app_error_unique_label() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::UniqueLabel, "duplicate label");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Conflict);
        assert_eq!(app.message.as_deref(), Some("duplicate label"));
    }

    #[test]
    fn turnkey_error_to_app_error_rate_limited() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "quota exceeded");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::RateLimited);
        assert_eq!(app.message.as_deref(), Some("quota exceeded"));
    }

    #[test]
    fn turnkey_error_to_app_error_timeout() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::Timeout, "request timed out");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Timeout);
        assert_eq!(app.message.as_deref(), Some("request timed out"));
    }

    #[test]
    fn turnkey_error_to_app_error_auth() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::Auth, "invalid credentials");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Unauthorized);
        assert_eq!(app.message.as_deref(), Some("invalid credentials"));
    }

    #[test]
    fn turnkey_error_to_app_error_network() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::Network, "connection failed");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Network);
        assert_eq!(app.message.as_deref(), Some("connection failed"));
    }

    #[test]
    fn turnkey_error_to_app_error_service() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::Service, "service error");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Turnkey);
        assert_eq!(app.message.as_deref(), Some("service error"));
    }

    #[test]
    fn turnkey_error_with_empty_message() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::Network, "");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Network);
        assert_eq!(app.message.as_deref(), Some(""));
    }

    #[test]
    fn turnkey_error_with_long_message() {
        let long_msg = "a".repeat(1000);
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::Service, &long_msg);
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Turnkey);
        assert_eq!(app.message.as_deref(), Some(long_msg.as_str()));
    }

    #[test]
    fn turnkey_error_with_unicode_message() {
        let turnkey = TurnkeyError::new(TurnkeyErrorKind::Auth, "Неверные учетные данные");
        let app: AppError = turnkey.into();
        assert_eq!(app.kind, AppErrorKind::Unauthorized);
        assert_eq!(app.message.as_deref(), Some("Неверные учетные данные"));
    }
}
