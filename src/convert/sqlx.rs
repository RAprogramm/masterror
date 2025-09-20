//! Conversions from `sqlx` errors into `AppError`.
//!
//! Feature flags:
//! - `sqlx`         → maps `sqlx_core::error::Error`
//! - `sqlx-migrate` → maps `sqlx::migrate::MigrateError`
//!
//! ## Mappings
//!
//! - `sqlx_core::error::Error::RowNotFound` → `AppErrorKind::NotFound`
//! - any other `sqlx_core::error::Error`    → `AppErrorKind::Database`
//! - `sqlx::migrate::MigrateError`          → `AppErrorKind::Database`
//!
//! The original error message is preserved in `AppError.message` for
//! observability. SQL driver–specific details are **not** mapped to separate
//! kinds to keep the taxonomy stable.
//!
//! ## Example
//!
//! ```rust,ignore
//! // Requires: features = ["sqlx"]
//! use masterror::{AppError, AppErrorKind};
//! use sqlx_core::error::Error as SqlxError;
//!
//! fn handle_db_error(e: SqlxError) -> AppError {
//!     e.into()
//! }
//!
//! // Simulated "row not found"
//! let err = handle_db_error(SqlxError::RowNotFound);
//! assert!(matches!(err.kind, AppErrorKind::NotFound));
//! ```

#[cfg(feature = "sqlx-migrate")]
use sqlx::migrate::MigrateError;
#[cfg(feature = "sqlx")]
use sqlx_core::error::Error as SqlxError;

#[cfg(any(feature = "sqlx", feature = "sqlx-migrate"))]
use crate::AppError;

/// Map a `sqlx_core::error::Error` into an `AppError`.
///
/// - `RowNotFound` → `AppErrorKind::NotFound`
/// - all other cases → `AppErrorKind::Database`
///
/// The database error message is preserved for debugging and log correlation.
#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => AppError::not_found("Record not found"),
            other => AppError::database(Some(other.to_string()))
        }
    }
}

/// Map a `sqlx::migrate::MigrateError` into an `AppError`.
///
/// All migration errors are considered `AppErrorKind::Database`.
/// The error string is preserved in `message`.
#[cfg(feature = "sqlx-migrate")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-migrate")))]
impl From<MigrateError> for AppError {
    fn from(err: MigrateError) -> Self {
        AppError::database(Some(err.to_string()))
    }
}

#[cfg(all(test, feature = "sqlx"))]
mod tests_sqlx {
    use std::io;

    use super::*;
    use crate::AppErrorKind;

    #[test]
    fn row_not_found_maps_to_not_found() {
        let err: AppError = SqlxError::RowNotFound.into();
        assert!(matches!(err.kind, AppErrorKind::NotFound));
    }

    #[test]
    fn other_error_maps_to_database() {
        // Prefer modern constructor; avoids clippy::io-other-error
        let io_err = io::Error::other("boom");
        let err: AppError = SqlxError::Io(io_err).into();
        assert!(matches!(err.kind, AppErrorKind::Database));
    }
}
