// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Turnkey-specific domain errors and error kind mappings.
//!
//! This module provides stable, high-level error categories for Turnkey
//! operations and their mapping to canonical [`AppErrorKind`] values.
//!
//! # Error Categories
//!
//! - [`TurnkeyErrorKind::UniqueLabel`] - Unique constraint violations
//! - [`TurnkeyErrorKind::RateLimited`] - Throttling or quota exceeded
//! - [`TurnkeyErrorKind::Timeout`] - Operation timeouts
//! - [`TurnkeyErrorKind::Auth`] - Authentication/authorization failures
//! - [`TurnkeyErrorKind::Network`] - Network-level errors
//! - [`TurnkeyErrorKind::Service`] - Generic Turnkey service errors
//!
//! # Mapping to AppErrorKind
//!
//! The mapping is intentionally conservative to maintain stability:
//!
//! | TurnkeyErrorKind | AppErrorKind |
//! |------------------|--------------|
//! | UniqueLabel      | Conflict     |
//! | RateLimited      | RateLimited  |
//! | Timeout          | Timeout      |
//! | Auth             | Unauthorized |
//! | Network          | Network      |
//! | Service          | Turnkey      |

use crate::{AppErrorKind, Error};

/// High-level, stable Turnkey error categories.
///
/// Marked `#[non_exhaustive]` to allow adding variants without a breaking
/// change. Consumers must use a wildcard arm when matching.
///
/// Mapping to [`AppErrorKind`] is intentionally conservative:
/// - `UniqueLabel` → `Conflict`
/// - `RateLimited` → `RateLimited`
/// - `Timeout` → `Timeout`
/// - `Auth` → `Unauthorized`
/// - `Network` → `Network`
/// - `Service` → `Turnkey`
///
/// # Examples
///
/// ```rust
/// use masterror::turnkey::TurnkeyErrorKind;
///
/// let kind = TurnkeyErrorKind::Timeout;
/// assert_eq!(kind.to_string(), "request timed out");
/// ```
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TurnkeyErrorKind {
    /// Unique label violation or duplicate resource.
    ///
    /// ```rust
    /// use masterror::turnkey::TurnkeyErrorKind;
    ///
    /// let kind = TurnkeyErrorKind::UniqueLabel;
    /// assert_eq!(kind.to_string(), "label already exists");
    /// ```
    #[error("label already exists")]
    UniqueLabel,

    /// Throttling or quota exceeded.
    ///
    /// ```rust
    /// use masterror::turnkey::TurnkeyErrorKind;
    ///
    /// let kind = TurnkeyErrorKind::RateLimited;
    /// assert_eq!(kind.to_string(), "rate limited or throttled");
    /// ```
    #[error("rate limited or throttled")]
    RateLimited,

    /// Operation exceeded allowed time.
    ///
    /// ```rust
    /// use masterror::turnkey::TurnkeyErrorKind;
    ///
    /// let kind = TurnkeyErrorKind::Timeout;
    /// assert_eq!(kind.to_string(), "request timed out");
    /// ```
    #[error("request timed out")]
    Timeout,

    /// Authentication/authorization failure.
    ///
    /// ```rust
    /// use masterror::turnkey::TurnkeyErrorKind;
    ///
    /// let kind = TurnkeyErrorKind::Auth;
    /// assert_eq!(kind.to_string(), "authentication/authorization failed");
    /// ```
    #[error("authentication/authorization failed")]
    Auth,

    /// Network-level error (DNS/connect/TLS/build).
    ///
    /// ```rust
    /// use masterror::turnkey::TurnkeyErrorKind;
    ///
    /// let kind = TurnkeyErrorKind::Network;
    /// assert_eq!(kind.to_string(), "network error");
    /// ```
    #[error("network error")]
    Network,

    /// Generic service error in the Turnkey subsystem.
    ///
    /// ```rust
    /// use masterror::turnkey::TurnkeyErrorKind;
    ///
    /// let kind = TurnkeyErrorKind::Service;
    /// assert_eq!(kind.to_string(), "service error");
    /// ```
    #[error("service error")]
    Service
}

/// Turnkey domain error with stable kind and safe, human-readable message.
///
/// Combines a [`TurnkeyErrorKind`] with a human-readable message.
/// Display format: `"{kind}: {msg}"`.
///
/// # Examples
///
/// ```rust
/// use masterror::turnkey::{TurnkeyError, TurnkeyErrorKind};
///
/// let err = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "quota exceeded");
/// assert_eq!(err.kind, TurnkeyErrorKind::RateLimited);
/// assert_eq!(err.msg, "quota exceeded");
/// assert_eq!(err.to_string(), "rate limited or throttled: quota exceeded");
/// ```
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{kind}: {msg}")]
pub struct TurnkeyError {
    /// Stable semantic category.
    pub kind: TurnkeyErrorKind,
    /// Public, non-sensitive message.
    pub msg:  String
}

impl TurnkeyError {
    /// Construct a new domain error.
    ///
    /// # Examples
    /// ```rust
    /// use masterror::turnkey::{TurnkeyError, TurnkeyErrorKind};
    /// let e = TurnkeyError::new(TurnkeyErrorKind::Timeout, "rpc deadline exceeded");
    /// assert!(matches!(e.kind, TurnkeyErrorKind::Timeout));
    /// ```
    #[inline]
    pub fn new(kind: TurnkeyErrorKind, msg: impl Into<String>) -> Self {
        Self {
            kind,
            msg: msg.into()
        }
    }
}

/// Map [`TurnkeyErrorKind`] into the canonical [`AppErrorKind`].
///
/// Keep mappings conservative and stable. See enum docs for rationale.
///
/// # Examples
///
/// ```rust
/// use masterror::{
///     AppErrorKind,
///     turnkey::{TurnkeyErrorKind, map_turnkey_kind}
/// };
///
/// assert_eq!(
///     map_turnkey_kind(TurnkeyErrorKind::Timeout),
///     AppErrorKind::Timeout
/// );
/// assert_eq!(
///     map_turnkey_kind(TurnkeyErrorKind::Auth),
///     AppErrorKind::Unauthorized
/// );
/// assert_eq!(
///     map_turnkey_kind(TurnkeyErrorKind::UniqueLabel),
///     AppErrorKind::Conflict
/// );
/// ```
#[must_use]
#[inline]
pub fn map_turnkey_kind(kind: TurnkeyErrorKind) -> AppErrorKind {
    match kind {
        TurnkeyErrorKind::UniqueLabel => AppErrorKind::Conflict,
        TurnkeyErrorKind::RateLimited => AppErrorKind::RateLimited,
        TurnkeyErrorKind::Timeout => AppErrorKind::Timeout,
        TurnkeyErrorKind::Auth => AppErrorKind::Unauthorized,
        TurnkeyErrorKind::Network => AppErrorKind::Network,
        TurnkeyErrorKind::Service => AppErrorKind::Turnkey,
        #[allow(unreachable_patterns)]
        _ => AppErrorKind::Turnkey
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turnkey_error_new_with_string() {
        let err = TurnkeyError::new(TurnkeyErrorKind::Timeout, "operation timed out".to_string());
        assert_eq!(err.kind, TurnkeyErrorKind::Timeout);
        assert_eq!(err.msg, "operation timed out");
    }

    #[test]
    fn turnkey_error_new_with_str() {
        let err = TurnkeyError::new(TurnkeyErrorKind::Network, "connection failed");
        assert_eq!(err.kind, TurnkeyErrorKind::Network);
        assert_eq!(err.msg, "connection failed");
    }

    #[test]
    fn turnkey_error_new_with_empty_message() {
        let err = TurnkeyError::new(TurnkeyErrorKind::Service, "");
        assert_eq!(err.kind, TurnkeyErrorKind::Service);
        assert_eq!(err.msg, "");
    }

    #[test]
    fn turnkey_error_new_with_long_message() {
        let long_msg = "a".repeat(1000);
        let err = TurnkeyError::new(TurnkeyErrorKind::Auth, &long_msg);
        assert_eq!(err.kind, TurnkeyErrorKind::Auth);
        assert_eq!(err.msg, long_msg);
    }

    #[test]
    fn turnkey_error_new_with_unicode() {
        let err = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "Превышена квота");
        assert_eq!(err.kind, TurnkeyErrorKind::RateLimited);
        assert_eq!(err.msg, "Превышена квота");
    }

    #[test]
    fn turnkey_error_display_format() {
        let err = TurnkeyError::new(TurnkeyErrorKind::UniqueLabel, "duplicate resource");
        assert_eq!(err.to_string(), "label already exists: duplicate resource");
    }

    #[test]
    fn turnkey_error_display_with_empty_message() {
        let err = TurnkeyError::new(TurnkeyErrorKind::Timeout, "");
        assert_eq!(err.to_string(), "request timed out: ");
    }

    #[test]
    fn turnkey_error_clone_creates_identical_copy() {
        let err1 = TurnkeyError::new(TurnkeyErrorKind::Network, "connection error");
        let err2 = err1.clone();
        assert_eq!(err1, err2);
        assert_eq!(err1.kind, err2.kind);
        assert_eq!(err1.msg, err2.msg);
    }

    #[test]
    fn turnkey_error_partial_eq_compares_kind_and_message() {
        let err1 = TurnkeyError::new(TurnkeyErrorKind::Auth, "invalid token");
        let err2 = TurnkeyError::new(TurnkeyErrorKind::Auth, "invalid token");
        let err3 = TurnkeyError::new(TurnkeyErrorKind::Auth, "different message");
        let err4 = TurnkeyError::new(TurnkeyErrorKind::Network, "invalid token");
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
        assert_ne!(err1, err4);
    }

    #[test]
    fn turnkey_error_kind_clone_works() {
        let kind1 = TurnkeyErrorKind::Timeout;
        let kind2 = kind1;
        assert_eq!(kind1, kind2);
    }

    #[test]
    fn turnkey_error_kind_partial_eq_works() {
        assert_eq!(TurnkeyErrorKind::UniqueLabel, TurnkeyErrorKind::UniqueLabel);
        assert_ne!(TurnkeyErrorKind::UniqueLabel, TurnkeyErrorKind::Timeout);
    }

    #[test]
    fn turnkey_error_kind_display_unique_label() {
        assert_eq!(
            TurnkeyErrorKind::UniqueLabel.to_string(),
            "label already exists"
        );
    }

    #[test]
    fn turnkey_error_kind_display_rate_limited() {
        assert_eq!(
            TurnkeyErrorKind::RateLimited.to_string(),
            "rate limited or throttled"
        );
    }

    #[test]
    fn turnkey_error_kind_display_timeout() {
        assert_eq!(TurnkeyErrorKind::Timeout.to_string(), "request timed out");
    }

    #[test]
    fn turnkey_error_kind_display_auth() {
        assert_eq!(
            TurnkeyErrorKind::Auth.to_string(),
            "authentication/authorization failed"
        );
    }

    #[test]
    fn turnkey_error_kind_display_network() {
        assert_eq!(TurnkeyErrorKind::Network.to_string(), "network error");
    }

    #[test]
    fn turnkey_error_kind_display_service() {
        assert_eq!(TurnkeyErrorKind::Service.to_string(), "service error");
    }

    #[test]
    fn map_turnkey_kind_unique_label() {
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::UniqueLabel),
            AppErrorKind::Conflict
        );
    }

    #[test]
    fn map_turnkey_kind_rate_limited() {
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::RateLimited),
            AppErrorKind::RateLimited
        );
    }

    #[test]
    fn map_turnkey_kind_timeout() {
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Timeout),
            AppErrorKind::Timeout
        );
    }

    #[test]
    fn map_turnkey_kind_auth() {
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Auth),
            AppErrorKind::Unauthorized
        );
    }

    #[test]
    fn map_turnkey_kind_network() {
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Network),
            AppErrorKind::Network
        );
    }

    #[test]
    fn map_turnkey_kind_service() {
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Service),
            AppErrorKind::Turnkey
        );
    }
}
