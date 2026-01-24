// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Modifier methods for `AppError` that change error properties.

use alloc::string::String;

use crate::{
    AppCode, RetryAdvice,
    app_error::core::{error::Error, types::MessageEditPolicy}
};

impl Error {
    /// Override the machine-readable [`AppCode`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppCode, AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::BadRequest, "test").with_code(AppCode::NotFound);
    /// assert_eq!(err.code, AppCode::NotFound);
    /// ```
    #[must_use]
    pub fn with_code(mut self, code: AppCode) -> Self {
        self.code = code;
        self.mark_dirty();
        self
    }

    /// Attach retry advice to the error.
    ///
    /// When mapped to HTTP, this becomes the `Retry-After` header.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::RateLimited, "slow down").with_retry_after_secs(60);
    /// assert_eq!(err.retry.map(|r| r.after_seconds), Some(60));
    /// ```
    #[must_use]
    pub fn with_retry_after_secs(mut self, secs: u64) -> Self {
        self.retry = Some(RetryAdvice {
            after_seconds: secs
        });
        self.mark_dirty();
        self
    }

    /// Attach a `WWW-Authenticate` challenge string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind};
    /// let err = AppError::new(AppErrorKind::Unauthorized, "auth required")
    ///     .with_www_authenticate("Bearer realm=\"api\"");
    /// assert!(err.www_authenticate.is_some());
    /// ```
    #[must_use]
    pub fn with_www_authenticate(mut self, value: impl Into<String>) -> Self {
        self.www_authenticate = Some(value.into());
        self.mark_dirty();
        self
    }

    /// Mark the message as redactable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, AppErrorKind, MessageEditPolicy};
    ///
    /// let err = AppError::new(AppErrorKind::Internal, "secret").redactable();
    /// assert_eq!(err.edit_policy, MessageEditPolicy::Redact);
    /// ```
    #[must_use]
    pub fn redactable(mut self) -> Self {
        self.edit_policy = MessageEditPolicy::Redact;
        self.mark_dirty();
        self
    }
}
