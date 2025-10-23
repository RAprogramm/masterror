// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Metadata attachment methods for [`ErrorResponse`].
//!
//! This module provides builder methods for attaching HTTP-specific metadata
//! to error responses, such as retry advice and authentication challenges.
//!
//! # Available Metadata
//!
//! - **Retry advice**: [`with_retry_after_secs`], [`with_retry_after_duration`]
//!   - Sets the `Retry-After` header in HTTP integrations
//! - **Authentication challenge**: [`with_www_authenticate`]
//!   - Sets the `WWW-Authenticate` header in HTTP integrations
//!
//! # Examples
//!
//! ```rust
//! use core::time::Duration;
//!
//! use masterror::{AppCode, ErrorResponse};
//!
//! let resp = ErrorResponse::new(503, AppCode::Internal, "service unavailable")
//!     .expect("status")
//!     .with_retry_after_secs(30)
//!     .with_www_authenticate("Bearer realm=\"api\"");
//!
//! assert_eq!(resp.retry.unwrap().after_seconds, 30);
//! assert_eq!(
//!     resp.www_authenticate.as_deref(),
//!     Some("Bearer realm=\"api\"")
//! );
//! ```
//!
//! [`with_retry_after_secs`]: ErrorResponse::with_retry_after_secs
//! [`with_retry_after_duration`]: ErrorResponse::with_retry_after_duration
//! [`with_www_authenticate`]: ErrorResponse::with_www_authenticate

use alloc::string::String;
use core::time::Duration;

use super::core::{ErrorResponse, RetryAdvice};

impl ErrorResponse {
    /// Attach retry advice (number of seconds).
    ///
    /// See [`with_retry_after_duration`](Self::with_retry_after_duration) for
    /// using a [`Duration`]. When present, integrations set the `Retry-After`
    /// header automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppCode, ErrorResponse};
    ///
    /// let resp = ErrorResponse::new(503, AppCode::Internal, "unavailable")
    ///     .expect("status")
    ///     .with_retry_after_secs(120);
    ///
    /// assert_eq!(resp.retry.expect("retry").after_seconds, 120);
    /// ```
    #[must_use]
    pub fn with_retry_after_secs(mut self, secs: u64) -> Self {
        self.retry = Some(RetryAdvice {
            after_seconds: secs
        });
        self
    }

    /// Attach retry advice as a [`Duration`].
    ///
    /// Equivalent to [`with_retry_after_secs`](Self::with_retry_after_secs).
    /// When present, integrations set the `Retry-After` header automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::time::Duration;
    ///
    /// use masterror::{AppCode, ErrorResponse};
    ///
    /// let resp = ErrorResponse::new(503, AppCode::Internal, "retry later")
    ///     .expect("status")
    ///     .with_retry_after_duration(Duration::from_secs(60));
    /// assert_eq!(resp.retry.expect("retry").after_seconds, 60);
    /// ```
    #[must_use]
    pub fn with_retry_after_duration(self, dur: Duration) -> Self {
        self.with_retry_after_secs(dur.as_secs())
    }

    /// Attach an authentication challenge string.
    ///
    /// When present, integrations set the `WWW-Authenticate` header
    /// automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppCode, ErrorResponse};
    ///
    /// let resp = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
    ///     .expect("status")
    ///     .with_www_authenticate("Bearer realm=\"api\"");
    ///
    /// assert_eq!(
    ///     resp.www_authenticate.as_deref(),
    ///     Some("Bearer realm=\"api\"")
    /// );
    /// ```
    #[must_use]
    pub fn with_www_authenticate(mut self, value: impl Into<String>) -> Self {
        self.www_authenticate = Some(value.into());
        self
    }
}
