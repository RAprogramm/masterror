// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::time::Duration;

use super::core::{ErrorResponse, RetryAdvice};

impl ErrorResponse {
    /// Attach retry advice (number of seconds).
    ///
    /// See [`with_retry_after_duration`](Self::with_retry_after_duration) for
    /// using a [`Duration`]. When present, integrations set the `Retry-After`
    /// header automatically.
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
    #[must_use]
    pub fn with_www_authenticate(mut self, value: impl Into<String>) -> Self {
        self.www_authenticate = Some(value.into());
        self
    }
}
