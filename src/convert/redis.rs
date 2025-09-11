//! Conversion from [`redis::RedisError`] into [`AppError`].
//!
//! Enabled with the `redis` feature flag.
//!
//! ## Mapping
//!
//! All Redis client errors are mapped to `AppErrorKind::Cache`.
//! The full error string from the driver is preserved in `message` for logs
//! and JSON payloads (if applicable).
//!
//! This categorization treats Redis as a cache infrastructure dependency.
//! If you need a different taxonomy (e.g. distinguishing caches from queues),
//! introduce dedicated `AppErrorKind` variants and adjust the mapping
//! accordingly.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppError, AppErrorKind};
//! use redis::RedisError;
//!
//! fn handle_cache_error(e: RedisError) -> AppError {
//!     e.into()
//! }
//!
//! // In production code, this would come from a Redis client operation
//! let dummy = RedisError::from((redis::ErrorKind::IoError, "connection lost"));
//! let app_err = handle_cache_error(dummy);
//!
//! assert!(matches!(app_err.kind, AppErrorKind::Cache));
//! ```

#[cfg(feature = "redis")]
use redis::RedisError;

#[cfg(feature = "redis")]
use crate::AppError;

/// Map any [`redis::RedisError`] into an [`AppError`] with kind `Cache`.
///
/// Rationale: Redis is treated as a backend cache dependency.
/// Detailed driver errors are kept in the message for diagnostics.
#[cfg(feature = "redis")]
#[cfg_attr(docsrs, doc(cfg(feature = "redis")))]
impl From<RedisError> for AppError {
    fn from(err: RedisError) -> Self {
        // Infrastructure cache issue -> cache-level error
        AppError::cache(format!("Redis error: {err}"))
    }
}

#[cfg(all(test, feature = "redis"))]
mod tests {
    use redis::ErrorKind;

    use super::*;
    use crate::AppErrorKind;

    #[test]
    fn maps_to_cache_kind() {
        let redis_err = RedisError::from((ErrorKind::IoError, "boom"));
        let app_err: AppError = redis_err.into();
        assert!(matches!(app_err.kind, AppErrorKind::Cache));
    }
}
