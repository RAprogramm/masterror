use std::borrow::Cow;

use super::core::AppError;
use crate::AppErrorKind;

impl AppError {
    // --- Canonical constructors (keep in sync with AppErrorKind) -------------

    // 4xx-ish
    /// Build a `NotFound` error.
    pub fn not_found(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::NotFound, msg)
    }
    /// Build a `Validation` error.
    pub fn validation(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Validation, msg)
    }
    /// Build an `Unauthorized` error.
    pub fn unauthorized(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Unauthorized, msg)
    }
    /// Build a `Forbidden` error.
    pub fn forbidden(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Forbidden, msg)
    }
    /// Build a `Conflict` error.
    pub fn conflict(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Conflict, msg)
    }
    /// Build a `BadRequest` error.
    pub fn bad_request(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::BadRequest, msg)
    }
    /// Build a `RateLimited` error.
    pub fn rate_limited(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::RateLimited, msg)
    }
    /// Build a `TelegramAuth` error.
    pub fn telegram_auth(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::TelegramAuth, msg)
    }

    // 5xx-ish
    /// Build an `Internal` error.
    pub fn internal(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Internal, msg)
    }
    /// Build a `Service` error (generic server-side service failure).
    pub fn service(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Service, msg)
    }
    /// Build a `Database` error with an optional message.
    ///
    /// Accepts `Option` to avoid gratuitous `.map(|...| ...)` at call sites
    /// when you may or may not have a safe-to-print string at hand.
    pub fn database(msg: Option<impl Into<Cow<'static, str>>>) -> Self {
        Self {
            kind:             AppErrorKind::Database,
            message:          msg.map(Into::into),
            retry:            None,
            www_authenticate: None
        }
    }
    /// Build a `Config` error.
    pub fn config(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Config, msg)
    }
    /// Build a `Turnkey` error.
    pub fn turnkey(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Turnkey, msg)
    }

    // Infra / network
    /// Build a `Timeout` error.
    pub fn timeout(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Timeout, msg)
    }
    /// Build a `Network` error.
    pub fn network(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Network, msg)
    }
    /// Build a `DependencyUnavailable` error.
    pub fn dependency_unavailable(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::DependencyUnavailable, msg)
    }
    /// Backward-compatible alias; routes to `DependencyUnavailable`.
    pub fn service_unavailable(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::DependencyUnavailable, msg)
    }

    // Serialization / external API / subsystems
    /// Build a `Serialization` error.
    pub fn serialization(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Serialization, msg)
    }
    /// Build a `Deserialization` error.
    pub fn deserialization(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Deserialization, msg)
    }
    /// Build an `ExternalApi` error.
    pub fn external_api(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::ExternalApi, msg)
    }
    /// Build a `Queue` error.
    pub fn queue(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Queue, msg)
    }
    /// Build a `Cache` error.
    pub fn cache(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::with(AppErrorKind::Cache, msg)
    }
}
