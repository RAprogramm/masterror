// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Control-flow macros for early returns with typed errors.
//!
//! These macros complement the typed [`AppError`](crate::AppError) APIs by
//! providing a lightweight, allocation-free way to short-circuit functions when
//! invariants are violated. Unlike the dynamic formatting helpers offered by
//! `anyhow` or `eyre`, the macros operate on pre-constructed error values so
//! the compiler keeps strong typing guarantees and no formatting work happens
//! on the success path.
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind, AppResult};
//!
//! fn guard(flag: bool) -> AppResult<()> {
//!     masterror::ensure!(flag, AppError::bad_request("flag must be true"));
//!     Ok(())
//! }
//!
//! assert!(guard(true).is_ok());
//! assert!(matches!(
//!     guard(false).unwrap_err().kind,
//!     AppErrorKind::BadRequest
//! ));
//! ```

/// Abort the enclosing function with an error when a condition fails.
///
/// The macro takes either a bare condition and error expression, or the more
/// explicit `cond = ..., else = ...` form. The error expression is evaluated
/// lazily only when the condition is false.
///
/// # Examples
///
/// Short-circuit a typed error:
///
/// ```rust
/// use masterror::{AppError, AppErrorKind, AppResult};
///
/// fn require(flag: bool) -> AppResult<()> {
///     masterror::ensure!(flag, AppError::bad_request("flag required"));
///     Ok(())
/// }
///
/// assert!(matches!(
///     require(false).unwrap_err().kind,
///     AppErrorKind::BadRequest
/// ));
/// ```
///
/// Use the verbose syntax for clarity in complex conditions:
///
/// ```rust
/// use masterror::{AppError, AppResult};
///
/// fn bounded(value: i32, max: i32) -> AppResult<()> {
///     masterror::ensure!(
///         cond = value <= max,
///         else = AppError::service("value too large")
///     );
///     Ok(())
/// }
///
/// assert!(bounded(2, 3).is_ok());
/// assert!(bounded(5, 3).is_err());
/// ```
#[macro_export]
macro_rules! ensure {
    (cond = $cond:expr, else = $err:expr $(,)?) => {
        $crate::ensure!($cond, $err)
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err($err);
        }
    };
}

/// Abort the enclosing function with the provided error.
///
/// This macro is a typed alternative to `anyhow::bail!`, delegating the
/// decision of how to construct the error to the caller. It never performs
/// formatting or allocations on the success path.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppError, AppErrorKind, AppResult};
///
/// fn reject() -> AppResult<()> {
///     masterror::fail!(AppError::unauthorized("token expired"));
/// }
///
/// let err = reject().unwrap_err();
/// assert!(matches!(err.kind, AppErrorKind::Unauthorized));
/// ```
#[macro_export]
macro_rules! fail {
    ($err:expr $(,)?) => {
        return Err($err);
    };
}
