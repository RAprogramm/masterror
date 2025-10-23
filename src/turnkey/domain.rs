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
        // Future-proofing: unknown variants map to Turnkey (500) by default.
        #[allow(unreachable_patterns)]
        _ => AppErrorKind::Turnkey
    }
}
