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
