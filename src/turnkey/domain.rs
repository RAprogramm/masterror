// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

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
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TurnkeyErrorKind {
    /// Unique label violation or duplicate resource.
    #[error("label already exists")]
    UniqueLabel,
    /// Throttling or quota exceeded.
    #[error("rate limited or throttled")]
    RateLimited,
    /// Operation exceeded allowed time.
    #[error("request timed out")]
    Timeout,
    /// Authentication/authorization failure.
    #[error("authentication/authorization failed")]
    Auth,
    /// Network-level error (DNS/connect/TLS/build).
    #[error("network error")]
    Network,
    /// Generic service error in the Turnkey subsystem.
    #[error("service error")]
    Service
}

/// Turnkey domain error with stable kind and safe, human-readable message.
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
