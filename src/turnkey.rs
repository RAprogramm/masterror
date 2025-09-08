//! Turnkey integration: error kinds, classification, and conversions.
//!
//! This module centralizes Turnkey-specific error taxonomy and mapping into
//! framework-agnostic [`AppError`] and [`AppErrorKind`].
//!
//! # Goals
//! - Stable domain kinds (`TurnkeyErrorKind`) decoupled from SDK texts.
//! - Conservative mapping to the canonical [`AppErrorKind`].
//! - Heuristic classifier for stringly-typed upstream errors.
//!
//! # Examples
//!
//! ```rust
//! use masterror::{
//!     AppError, AppErrorKind,
//!     turnkey::{TurnkeyError, TurnkeyErrorKind, classify_turnkey_error}
//! };
//!
//! // Construct a domain error
//! let e = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "429 from upstream");
//!
//! // Convert into AppError for transport mapping
//! let app: AppError = e.clone().into();
//! assert_eq!(app.kind, AppErrorKind::RateLimited);
//!
//! // Classify raw SDK message
//! let k = classify_turnkey_error("label must be unique");
//! assert!(matches!(k, TurnkeyErrorKind::UniqueLabel));
//! ```

use thiserror::Error;

use crate::{AppError, AppErrorKind};

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

/// Heuristic classifier for raw SDK/provider messages (ASCII case-insensitive).
///
/// This helper **does not allocate**; it performs case-insensitive `contains`
/// checks over the input string to map common upstream texts to stable kinds.
///
/// The classifier is intentionally minimal; providers can and will change
/// messages. Prefer returning structured errors from adapters whenever
/// possible.
///
/// # Examples
/// ```rust
/// use masterror::turnkey::{TurnkeyErrorKind, classify_turnkey_error};
/// assert!(matches!(
///     classify_turnkey_error("429 Too Many Requests"),
///     TurnkeyErrorKind::RateLimited
/// ));
/// assert!(matches!(
///     classify_turnkey_error("label must be unique"),
///     TurnkeyErrorKind::UniqueLabel
/// ));
/// assert!(matches!(
///     classify_turnkey_error("request timed out"),
///     TurnkeyErrorKind::Timeout
/// ));
/// ```
#[must_use]
pub fn classify_turnkey_error(msg: &str) -> TurnkeyErrorKind {
    // Patterns grouped by kind. Keep short, ASCII, and conservative.
    const UNIQUE_PATTERNS: &[&str] = &[
        "label must be unique",
        "already exists",
        "duplicate",
        "unique"
    ];
    const RL_PATTERNS: &[&str] = &["429", "rate", "throttle"];
    const TO_PATTERNS: &[&str] = &["timeout", "timed out", "deadline exceeded"];
    const AUTH_PATTERNS: &[&str] = &["401", "403", "unauthor", "forbidden"];
    const NET_PATTERNS: &[&str] = &["network", "connection", "connect", "dns", "tls", "socket"];

    if contains_any_nocase(msg, UNIQUE_PATTERNS) {
        TurnkeyErrorKind::UniqueLabel
    } else if contains_any_nocase(msg, RL_PATTERNS) {
        TurnkeyErrorKind::RateLimited
    } else if contains_any_nocase(msg, TO_PATTERNS) {
        TurnkeyErrorKind::Timeout
    } else if contains_any_nocase(msg, AUTH_PATTERNS) {
        TurnkeyErrorKind::Auth
    } else if contains_any_nocase(msg, NET_PATTERNS) {
        TurnkeyErrorKind::Network
    } else {
        TurnkeyErrorKind::Service
    }
}

/// ASCII case-insensitive `haystack.contains(needle)` without allocation.
/// comments in English
#[inline]
fn contains_nocase(haystack: &str, needle: &str) -> bool {
    // Fast path: empty needle always matches.
    if needle.is_empty() {
        return true;
    }
    // Walk haystack windows and compare ASCII case-insensitively.
    haystack.as_bytes().windows(needle.len()).any(|w| {
        w.iter()
            .copied()
            .map(ascii_lower)
            .eq(needle.as_bytes().iter().copied().map(ascii_lower))
    })
}

/// Check whether `haystack` contains any of the `needles` (ASCII
/// case-insensitive).
#[inline]
fn contains_any_nocase(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| contains_nocase(haystack, n))
}

/// Lowercase for ASCII bytes only; leaves non-ASCII untouched.
/// comments in English
#[inline]
const fn ascii_lower(b: u8) -> u8 {
    // ASCII-only fold without RangeInclusive to keep const-friendly on MSRV 1.89
    if b >= b'A' && b <= b'Z' { b + 32 } else { b }
}

// ── Conversions into AppError ────────────────────────────────────────────────

impl From<TurnkeyErrorKind> for AppErrorKind {
    #[inline]
    fn from(k: TurnkeyErrorKind) -> Self {
        map_turnkey_kind(k)
    }
}

impl From<TurnkeyError> for AppError {
    #[inline]
    fn from(e: TurnkeyError) -> Self {
        // Prefer explicit constructors to keep transport mapping consistent.
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
    use crate::AppErrorKind;

    #[test]
    fn map_is_stable() {
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::UniqueLabel),
            AppErrorKind::Conflict
        );
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::RateLimited),
            AppErrorKind::RateLimited
        );
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Timeout),
            AppErrorKind::Timeout
        );
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Auth),
            AppErrorKind::Unauthorized
        );
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Network),
            AppErrorKind::Network
        );
        assert_eq!(
            map_turnkey_kind(TurnkeyErrorKind::Service),
            AppErrorKind::Turnkey
        );
    }

    #[test]
    fn classifier_unique() {
        for s in [
            "Label must be UNIQUE",
            "already exists: trading-key-foo",
            "duplicate label",
            "unique constraint violation"
        ] {
            assert!(
                matches!(classify_turnkey_error(s), TurnkeyErrorKind::UniqueLabel),
                "failed on: {s}"
            );
        }
    }

    #[test]
    fn classifier_rate_limited() {
        for s in [
            "429 Too Many Requests",
            "rate limit exceeded",
            "throttled by upstream"
        ] {
            assert!(
                matches!(classify_turnkey_error(s), TurnkeyErrorKind::RateLimited),
                "failed on: {s}"
            );
        }
    }

    #[test]
    fn classifier_timeout() {
        for s in [
            "request timed out",
            "Timeout while waiting",
            "deadline exceeded"
        ] {
            assert!(
                matches!(classify_turnkey_error(s), TurnkeyErrorKind::Timeout),
                "failed on: {s}"
            );
        }
    }

    #[test]
    fn classifier_auth() {
        for s in ["401 Unauthorized", "403 Forbidden", "unauthor ized"] {
            assert!(
                matches!(classify_turnkey_error(s), TurnkeyErrorKind::Auth),
                "failed on: {s}"
            );
        }
    }

    #[test]
    fn classifier_network() {
        for s in [
            "network error",
            "connection reset",
            "DNS failure",
            "TLS handshake",
            "socket hang up"
        ] {
            assert!(
                matches!(classify_turnkey_error(s), TurnkeyErrorKind::Network),
                "failed on: {s}"
            );
        }
    }

    #[test]
    fn contains_nocase_works_without_alloc() {
        assert!(contains_nocase("ABCdef", "cDe"));
        assert!(contains_any_nocase("hello world", &["nope", "WORLD"]));
        assert!(!contains_nocase("rustacean", "python"));
        assert!(contains_nocase("", "")); // by definition
    }

    #[test]
    fn from_turnkey_error_into_app_error() {
        let e = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "try later");
        let a: AppError = e.into();
        assert_eq!(a.kind, AppErrorKind::RateLimited);
        // message plumbing is AppError-specific; sanity-check only kind here.
    }
}
