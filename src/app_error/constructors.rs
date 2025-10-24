// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Canonical constructors for [`AppError`].
//!
//! This module provides ergonomic constructors for all error kinds, organized
//! by category:
//!
//! - **4xx-ish (Client errors)**: `not_found`, `validation`, `unauthorized`,
//!   `forbidden`, `conflict`, `bad_request`, `rate_limited`, `telegram_auth`
//! - **5xx-ish (Server errors)**: `internal`, `service`, `database`, `config`,
//!   `turnkey`
//! - **Infra/Network**: `timeout`, `network`, `dependency_unavailable`,
//!   `service_unavailable`
//! - **Serialization/External**: `serialization`, `deserialization`,
//!   `external_api`, `queue`, `cache`
//!
//! All constructors accept any type that implements `Into<Cow<'static, str>>`,
//! enabling flexible message construction from string literals, owned strings,
//! or pre-built `Cow` instances.

use alloc::borrow::Cow;

use super::core::AppError;
use crate::AppErrorKind;

impl AppError {
    // --- Canonical constructors (keep in sync with AppErrorKind) -------------

    // 4xx-ish
    /// Build a `NotFound` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::not_found("user not found");
    /// assert_eq!(err.message.as_deref(), Some("user not found"));
    /// ```
    pub fn not_found(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::NotFound, msg)
    }

    /// Build a `Validation` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::validation("invalid email format");
    /// assert_eq!(err.message.as_deref(), Some("invalid email format"));
    /// ```
    pub fn validation(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Validation, msg)
    }

    /// Build an `Unauthorized` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::unauthorized("missing authentication token");
    /// assert_eq!(err.message.as_deref(), Some("missing authentication token"));
    /// ```
    pub fn unauthorized(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Unauthorized, msg)
    }

    /// Build a `Forbidden` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::forbidden("insufficient permissions");
    /// assert_eq!(err.message.as_deref(), Some("insufficient permissions"));
    /// ```
    pub fn forbidden(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Forbidden, msg)
    }

    /// Build a `Conflict` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::conflict("resource already exists");
    /// assert_eq!(err.message.as_deref(), Some("resource already exists"));
    /// ```
    pub fn conflict(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Conflict, msg)
    }

    /// Build a `BadRequest` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::bad_request("malformed JSON payload");
    /// assert_eq!(err.message.as_deref(), Some("malformed JSON payload"));
    /// ```
    pub fn bad_request(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::BadRequest, msg)
    }

    /// Build a `RateLimited` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::rate_limited("rate limit exceeded");
    /// assert_eq!(err.message.as_deref(), Some("rate limit exceeded"));
    /// ```
    pub fn rate_limited(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::RateLimited, msg)
    }

    /// Build a `TelegramAuth` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::telegram_auth("invalid telegram signature");
    /// assert_eq!(err.message.as_deref(), Some("invalid telegram signature"));
    /// ```
    pub fn telegram_auth(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::TelegramAuth, msg)
    }

    // 5xx-ish
    /// Build an `Internal` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::internal("unexpected server error");
    /// assert_eq!(err.message.as_deref(), Some("unexpected server error"));
    /// ```
    pub fn internal(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Internal, msg)
    }

    /// Build a `Service` error (generic server-side service failure).
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::service("service processing failed");
    /// assert_eq!(err.message.as_deref(), Some("service processing failed"));
    /// ```
    pub fn service(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Service, msg)
    }
    /// Build a `Database` error with an optional message.
    ///
    /// This constructor accepts a pre-built [`Cow`] so callers that already
    /// manage ownership can pass either borrowed or owned strings. When you
    /// have plain string data, prefer [`AppError::database_with_message`].
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::database(None);
    /// assert!(err.message.is_none());
    /// ```
    pub fn database(msg: Option<Cow<'static, str>>) -> Self {
        let err = Self::new_raw(AppErrorKind::Database, msg);
        err.emit_telemetry();
        err
    }

    /// Build a `Database` error with a message.
    ///
    /// Convenience wrapper around [`AppError::database`] for the common case
    /// where you start from a plain string-like value.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::database_with_message("db down");
    /// assert_eq!(err.message.as_deref(), Some("db down"));
    /// ```
    pub fn database_with_message(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::database(Some(msg.into()))
    }
    /// Build a `Config` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::config("missing required configuration key");
    /// assert_eq!(
    ///     err.message.as_deref(),
    ///     Some("missing required configuration key")
    /// );
    /// ```
    pub fn config(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Config, msg)
    }

    /// Build a `Turnkey` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::turnkey("turnkey operation failed");
    /// assert_eq!(err.message.as_deref(), Some("turnkey operation failed"));
    /// ```
    pub fn turnkey(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Turnkey, msg)
    }

    // Infra / network
    /// Build a `Timeout` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::timeout("request timed out after 30s");
    /// assert_eq!(err.message.as_deref(), Some("request timed out after 30s"));
    /// ```
    pub fn timeout(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Timeout, msg)
    }

    /// Build a `Network` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::network("connection refused");
    /// assert_eq!(err.message.as_deref(), Some("connection refused"));
    /// ```
    pub fn network(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Network, msg)
    }

    /// Build a `DependencyUnavailable` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::dependency_unavailable("payment service unavailable");
    /// assert_eq!(err.message.as_deref(), Some("payment service unavailable"));
    /// ```
    pub fn dependency_unavailable(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::DependencyUnavailable, msg)
    }

    /// Backward-compatible alias; routes to `DependencyUnavailable`.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::service_unavailable("service temporarily unavailable");
    /// assert_eq!(
    ///     err.message.as_deref(),
    ///     Some("service temporarily unavailable")
    /// );
    /// ```
    pub fn service_unavailable(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::DependencyUnavailable, msg)
    }

    // Serialization / external API / subsystems
    /// Build a `Serialization` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::serialization("failed to serialize response");
    /// assert_eq!(err.message.as_deref(), Some("failed to serialize response"));
    /// ```
    pub fn serialization(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Serialization, msg)
    }

    /// Build a `Deserialization` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::deserialization("failed to parse JSON");
    /// assert_eq!(err.message.as_deref(), Some("failed to parse JSON"));
    /// ```
    pub fn deserialization(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Deserialization, msg)
    }

    /// Build an `ExternalApi` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::external_api("third-party API returned error");
    /// assert_eq!(
    ///     err.message.as_deref(),
    ///     Some("third-party API returned error")
    /// );
    /// ```
    pub fn external_api(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::ExternalApi, msg)
    }

    /// Build a `Queue` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::queue("queue is full");
    /// assert_eq!(err.message.as_deref(), Some("queue is full"));
    /// ```
    pub fn queue(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Queue, msg)
    }

    /// Build a `Cache` error.
    ///
    /// ```rust
    /// use masterror::AppError;
    ///
    /// let err = AppError::cache("cache lookup failed");
    /// assert_eq!(err.message.as_deref(), Some("cache lookup failed"));
    /// ```
    pub fn cache(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Cache, msg)
    }
}
