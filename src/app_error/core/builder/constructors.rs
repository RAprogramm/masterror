// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Core constructors for creating `AppError` instances.

use alloc::borrow::Cow;

use crate::{AppErrorKind, app_error::core::error::Error};

impl Error {
    /// Create a new [`Error`] with a kind and message.
    ///
    /// This is equivalent to [`Error::with`], provided for API symmetry and to
    /// keep doctests readable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::BadRequest, "invalid payload");
    /// assert!(err.message.is_some());
    /// ```
    #[must_use]
    pub fn new(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(kind, msg)
    }

    /// Create an error with the given kind and message.
    ///
    /// Prefer named helpers (e.g. [`Error::not_found`]) where it clarifies
    /// intent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::with(AppErrorKind::Validation, "bad input");
    /// assert_eq!(err.kind, AppErrorKind::Validation);
    /// ```
    #[must_use]
    pub fn with(kind: AppErrorKind, msg: impl Into<Cow<'static, str>>) -> Self {
        let err = Self::new_raw(kind, Some(msg.into()));
        err.emit_telemetry();
        err
    }

    /// Create a message-less error with the given kind.
    ///
    /// Useful when the kind alone conveys sufficient information to the client.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::bare(AppErrorKind::NotFound);
    /// assert!(err.message.is_none());
    /// ```
    #[must_use]
    pub fn bare(kind: AppErrorKind) -> Self {
        let err = Self::new_raw(kind, None);
        err.emit_telemetry();
        err
    }
}
