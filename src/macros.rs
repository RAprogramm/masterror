// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Control-flow and construction macros for typed errors.
//!
//! The control-flow macros ([`ensure!`](crate::ensure) and
//! [`fail!`](crate::fail)) complement the typed [`AppError`](crate::AppError)
//! APIs by providing a lightweight, allocation-free way to short-circuit
//! functions when invariants are violated. Unlike the dynamic formatting
//! helpers offered by `anyhow` or `eyre`, they operate on pre-constructed
//! error values so the compiler keeps strong typing guarantees and no
//! formatting work happens on the success path.
//!
//! The expression macro [`app_error!`](crate::app_error) covers the ad-hoc
//! construction side: it builds an [`AppError`](crate::AppError) from a kind
//! and an optional format-style message, mirroring `anyhow::anyhow!` while
//! staying inside the typed taxonomy.
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
///
/// Clippy format-args lints (such as `clippy::uninlined_format_args`) see
/// through this macro and lint format arguments passed inside it.
#[macro_export]
#[clippy::format_args]
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
///
/// Clippy format-args lints (such as `clippy::uninlined_format_args`) see
/// through this macro and lint format arguments passed inside it.
#[macro_export]
#[clippy::format_args]
macro_rules! fail {
    ($err:expr $(,)?) => {
        return Err($err);
    };
}

/// Construct an [`AppError`](crate::AppError) expression from a kind and an
/// optional format-style message.
///
/// This macro is the typed counterpart of `anyhow::anyhow!`. It evaluates to
/// an [`AppError`](crate::AppError) value, so it can be used anywhere an
/// expression is expected: `ok_or_else`, `map_err`, `return Err(...)` or as
/// the argument to [`fail!`](crate::fail).
///
/// Two forms are supported:
///
/// - `app_error!(kind)` expands to [`AppError::bare`](crate::AppError::bare)
///   and performs no allocation.
/// - `app_error!(kind, "format {args}")` expands to
///   [`AppError::with`](crate::AppError::with) with a message built by
///   [`format!`](alloc::format), including implicit named-argument capture.
///   Exactly one allocation happens, driven by `format_args!`.
///
/// # Examples
///
/// Kind-only, allocation-free:
///
/// ```rust
/// use masterror::{AppErrorKind, app_error};
///
/// let err = app_error!(AppErrorKind::Timeout);
/// assert!(matches!(err.kind, AppErrorKind::Timeout));
/// assert!(err.message.is_none());
/// ```
///
/// Formatted message with implicit capture:
///
/// ```rust
/// use masterror::{AppErrorKind, app_error};
///
/// let value = 42;
/// let err = app_error!(AppErrorKind::Validation, "bad value: {value}");
/// assert!(matches!(err.kind, AppErrorKind::Validation));
/// assert_eq!(err.message.as_deref(), Some("bad value: 42"));
/// ```
///
/// Expression position and composition with [`fail!`](crate::fail):
///
/// ```rust
/// use masterror::{AppErrorKind, AppResult, app_error};
///
/// fn find(id: u64) -> AppResult<u64> {
///     let found = None::<u64>;
///     let value = found.ok_or_else(|| app_error!(AppErrorKind::NotFound, "no entity {id}"))?;
///     Ok(value)
/// }
///
/// fn reject() -> AppResult<()> {
///     masterror::fail!(app_error!(AppErrorKind::Unauthorized, "token expired"));
/// }
///
/// assert!(matches!(find(7).unwrap_err().kind, AppErrorKind::NotFound));
/// assert!(matches!(
///     reject().unwrap_err().kind,
///     AppErrorKind::Unauthorized
/// ));
/// ```
///
/// Clippy format-args lints (such as `clippy::uninlined_format_args`) see
/// through this macro and lint the format string and arguments directly.
#[macro_export]
#[clippy::format_args]
macro_rules! app_error {
    ($kind:expr $(,)?) => {
        $crate::AppError::bare($kind)
    };
    ($kind:expr, $($fmt:tt)+) => {
        $crate::AppError::with($kind, $crate::__private::format!($($fmt)+))
    };
}
