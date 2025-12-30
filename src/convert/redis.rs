// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion from [`redis::RedisError`] into [`Error`].
//!
//! Enabled with the `redis` feature flag.
//!
//! ## Mapping
//!
//! All Redis client errors are mapped to `AppErrorKind::Cache` by default and
//! enriched with structured metadata (error kind, code, retry hints). Timeout
//! and infrastructure-level failures are promoted to `Timeout` or
//! `DependencyUnavailable` respectively. Metadata captures cluster redirects,
//! retry strategy and low-level flags without exposing sensitive payloads.
//!
//! This categorization treats Redis as a cache infrastructure dependency.
//! If you need a different taxonomy (e.g. distinguishing caches from queues),
//! introduce dedicated `AppErrorKind` variants and adjust the mapping
//! accordingly.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppErrorKind, Error};
//! use redis::RedisError;
//!
//! fn handle_cache_error(e: RedisError) -> Error {
//!     e.into()
//! }
//!
//! // In production code, this would come from a Redis client operation
//! let dummy = RedisError::from((redis::ErrorKind::Io, "connection lost"));
//! let app_err = handle_cache_error(dummy);
//!
//! assert!(matches!(app_err.kind, AppErrorKind::Cache));
//! ```

#[cfg(feature = "redis")]
use redis::{ErrorKind, RedisError, RetryMethod, ServerErrorKind};

#[cfg(feature = "redis")]
use crate::{AppErrorKind, Context, Error, field};

/// Map any [`redis::RedisError`] into an [`crate::AppError`] with kind `Cache`.
///
/// Rationale: Redis is treated as a backend cache dependency.
/// Detailed driver errors are kept in the message for diagnostics.
#[cfg(feature = "redis")]
#[cfg_attr(docsrs, doc(cfg(feature = "redis")))]
impl From<RedisError> for Error {
    fn from(err: RedisError) -> Self {
        let (context, retry_after) = build_context(&err);
        let mut error = context.into_error(err);
        if let Some(secs) = retry_after {
            error = error.with_retry_after_secs(secs);
        }
        error
    }
}

#[cfg(feature = "redis")]
fn build_context(err: &RedisError) -> (Context, Option<u64>) {
    let mut context = Context::new(AppErrorKind::Cache)
        .with(field::str("redis.kind", format!("{:?}", err.kind())))
        .with(field::str("redis.category", err.category().to_owned()))
        .with(field::bool("redis.is_timeout", err.is_timeout()))
        .with(field::bool(
            "redis.is_cluster_error",
            err.is_cluster_error()
        ))
        .with(field::bool(
            "redis.is_connection_refused",
            err.is_connection_refusal()
        ))
        .with(field::bool(
            "redis.is_connection_dropped",
            err.is_connection_dropped()
        ));
    if let Some(code) = err.code() {
        context = context.with(field::str("redis.code", code.to_owned()));
    }
    if err.is_timeout() {
        context = context.category(AppErrorKind::Timeout);
    } else if err.is_connection_refusal()
        || err.is_connection_dropped()
        || err.is_cluster_error()
        || err.is_io_error()
        || is_busy_loading(err)
    {
        context = context.category(AppErrorKind::DependencyUnavailable);
    }
    if let Some((addr, slot)) = err.redirect_node() {
        context = context
            .with(field::str("redis.redirect_addr", addr.to_owned()))
            .with(field::u64("redis.redirect_slot", u64::from(slot)));
    }
    let (retry_method_label, retry_after) = retry_method_details(err.retry_method());
    context = context.with(field::str("redis.retry_method", retry_method_label));
    if let Some(secs) = retry_after {
        context = context.with(field::u64("redis.retry_after_hint_secs", secs));
    }
    (context, retry_after)
}

#[cfg(feature = "redis")]
fn is_busy_loading(err: &RedisError) -> bool {
    err.kind() == ErrorKind::Server(ServerErrorKind::BusyLoading)
}

#[cfg(feature = "redis")]
const fn retry_method_details(method: RetryMethod) -> (&'static str, Option<u64>) {
    match method {
        RetryMethod::NoRetry => ("NoRetry", None),
        RetryMethod::RetryImmediately => ("RetryImmediately", Some(0)),
        RetryMethod::AskRedirect => ("AskRedirect", Some(0)),
        RetryMethod::MovedRedirect => ("MovedRedirect", Some(0)),
        RetryMethod::Reconnect => ("Reconnect", Some(1)),
        RetryMethod::ReconnectFromInitialConnections => {
            ("ReconnectFromInitialConnections", Some(1))
        }
        RetryMethod::WaitAndRetry => ("WaitAndRetry", Some(2)),
        _ => ("Other", None)
    }
}

#[cfg(all(test, feature = "redis"))]
mod tests {
    use redis::ErrorKind;

    use super::*;
    use crate::{AppErrorKind, FieldValue};

    #[test]
    fn maps_io_error_to_dependency_unavailable() {
        let redis_err = RedisError::from((ErrorKind::Io, "boom"));
        let app_err: Error = redis_err.into();
        assert!(matches!(app_err.kind, AppErrorKind::DependencyUnavailable));
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("redis.kind"),
            Some(&FieldValue::Str("Io".into()))
        );
    }

    #[test]
    fn maps_client_error_to_cache() {
        let redis_err = RedisError::from((ErrorKind::Client, "bad config"));
        let app_err: Error = redis_err.into();
        assert!(matches!(app_err.kind, AppErrorKind::Cache));
    }
}
