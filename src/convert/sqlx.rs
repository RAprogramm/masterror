//! Conversions from `sqlx` errors into [`Error`].
//!
//! Feature flags:
//! - `sqlx`         → maps `sqlx_core::error::Error`
//! - `sqlx-migrate` → maps `sqlx::migrate::MigrateError`
//!
//! ## Mappings
//!
//! - `sqlx_core::error::Error::RowNotFound` → `AppErrorKind::NotFound`
//! - Database constraint errors capture SQLSTATE/constraint metadata and map to
//!   `Conflict`/`Validation`
//! - Transient SQLSTATEs (e.g. `40001`, `55P03`) attach retry hints
//! - `sqlx::migrate::MigrateError` → `AppErrorKind::Database` with migration
//!   phase metadata
//!
//! Structured metadata includes SQLSTATE codes, constraint names and migration
//! phases to aid observability while keeping secrets out of public payloads.
//! Known SQLSTATE codes override [`AppCode`] (`UNIQUE_VIOLATION` →
//! `USER_ALREADY_EXISTS`).
//!
//! ## Example
//!
//! ```rust,ignore
//! // Requires: features = ["sqlx"]
//! use masterror::{AppErrorKind, Error};
//! use sqlx_core::error::Error as SqlxError;
//!
//! fn handle_db_error(e: SqlxError) -> Error {
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
use sqlx_core::error::{DatabaseError, Error as SqlxError, ErrorKind as SqlxErrorKind};

#[cfg(any(feature = "sqlx", feature = "sqlx-migrate"))]
use crate::{AppCode, AppErrorKind, Context, Error, field};

#[cfg(feature = "sqlx")]
const SQLSTATE_CODE_OVERRIDES: &[(&str, AppCode)] = &[
    ("23505", AppCode::UserAlreadyExists),
    ("23503", AppCode::Conflict),
    ("23502", AppCode::Validation),
    ("23514", AppCode::Validation)
];

#[cfg(feature = "sqlx")]
const SQLSTATE_RETRY_HINTS: &[(&str, u64)] = &[("40001", 1), ("55P03", 1)];

/// Map a `sqlx_core::error::Error` into [`Error`].
///
/// - `RowNotFound` → `AppErrorKind::NotFound`
/// - database constraint errors attach SQLSTATE and constraint metadata
/// - concurrency SQLSTATEs attach retry hints
#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Self {
        let (context, retry_after) = build_sqlx_context(&err);
        let mut error = context.into_error(err);
        if let Some(secs) = retry_after {
            error = error.with_retry_after_secs(secs);
        }
        error
    }
}

/// Map a `sqlx::migrate::MigrateError` into [`Error`].
///
/// Errors are categorised as `Database` with metadata describing the failing
/// migration phase.
#[cfg(feature = "sqlx-migrate")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-migrate")))]
impl From<MigrateError> for Error {
    fn from(err: MigrateError) -> Self {
        build_migrate_context(&err).into_error(err)
    }
}

#[cfg(feature = "sqlx")]
fn build_sqlx_context(err: &SqlxError) -> (Context, Option<u64>) {
    let (mut context, retry_after) = match err {
        SqlxError::RowNotFound => (
            Context::new(AppErrorKind::NotFound).with(field::str("db.reason", "row_not_found")),
            None
        ),
        SqlxError::Database(db_err) => classify_database_error(db_err.as_ref()),
        SqlxError::Io(io_err) => (
            Context::new(AppErrorKind::DependencyUnavailable)
                .with(field::str("db.reason", "io_error"))
                .with(field::str("io.kind", format!("{:?}", io_err.kind()))),
            None
        ),
        SqlxError::PoolTimedOut => (
            Context::new(AppErrorKind::Timeout).with(field::str("db.reason", "pool_timeout")),
            Some(1)
        ),
        SqlxError::PoolClosed => (
            Context::new(AppErrorKind::DependencyUnavailable)
                .with(field::str("db.reason", "pool_closed")),
            None
        ),
        SqlxError::WorkerCrashed => (
            Context::new(AppErrorKind::DependencyUnavailable)
                .with(field::str("db.reason", "worker_crashed")),
            Some(1)
        ),
        SqlxError::Configuration(source) => (
            Context::new(AppErrorKind::Config)
                .with(field::str("db.reason", "configuration"))
                .with(field::str("db.detail", source.to_string())),
            None
        ),
        SqlxError::InvalidArgument(message) => (
            Context::new(AppErrorKind::BadRequest)
                .with(field::str("db.reason", "invalid_argument"))
                .with(field::str("db.argument", message.clone())),
            None
        ),
        SqlxError::ColumnDecode {
            index, ..
        } => (
            Context::new(AppErrorKind::Deserialization)
                .with(field::str("db.reason", "column_decode"))
                .with(field::str("db.column", index.clone())),
            None
        ),
        SqlxError::ColumnNotFound(name) => (
            Context::new(AppErrorKind::Internal)
                .with(field::str("db.reason", "column_not_found"))
                .with(field::str("db.column", name.clone())),
            None
        ),
        SqlxError::ColumnIndexOutOfBounds {
            index,
            len
        } => (
            Context::new(AppErrorKind::Internal)
                .with(field::str("db.reason", "column_index_out_of_bounds"))
                .with(field::u64("db.index", *index as u64))
                .with(field::u64("db.len", *len as u64)),
            None
        ),
        SqlxError::TypeNotFound {
            type_name
        } => (
            Context::new(AppErrorKind::Internal)
                .with(field::str("db.reason", "type_not_found"))
                .with(field::str("db.type", type_name.clone())),
            None
        ),
        SqlxError::Encode(_) => (
            Context::new(AppErrorKind::Serialization).with(field::str("db.reason", "encode")),
            None
        ),
        SqlxError::Decode(_) => (
            Context::new(AppErrorKind::Deserialization).with(field::str("db.reason", "decode")),
            None
        ),
        SqlxError::Protocol(detail) => (
            Context::new(AppErrorKind::DependencyUnavailable)
                .with(field::str("db.reason", "protocol"))
                .with(field::str("db.detail", detail.clone())),
            Some(1)
        ),
        SqlxError::Tls(_) => (
            Context::new(AppErrorKind::Network).with(field::str("db.reason", "tls")),
            Some(1)
        ),
        SqlxError::AnyDriverError(_) => (
            Context::new(AppErrorKind::Database).with(field::str("db.reason", "driver_error")),
            None
        ),
        SqlxError::InvalidSavePointStatement => (
            Context::new(AppErrorKind::Internal)
                .with(field::str("db.reason", "invalid_savepoint")),
            None
        ),
        SqlxError::BeginFailed => (
            Context::new(AppErrorKind::DependencyUnavailable)
                .with(field::str("db.reason", "begin_failed")),
            Some(1)
        ),
        other => (
            Context::new(AppErrorKind::Database)
                .with(field::str("db.reason", "unclassified"))
                .with(field::str("db.detail", format!("{:?}", other))),
            None
        )
    };

    if let Some(secs) = retry_after {
        context = context.with(field::u64("db.retry_after_hint_secs", secs));
    }

    (context, retry_after)
}

#[cfg(feature = "sqlx")]
fn classify_database_error(error: &(dyn DatabaseError + 'static)) -> (Context, Option<u64>) {
    let mut context = Context::new(AppErrorKind::Database)
        .with(field::str("db.reason", "database_error"))
        .with(field::str("db.message", error.message().to_owned()));

    if let Some(constraint) = error.constraint() {
        context = context.with(field::str("db.constraint", constraint.to_owned()));
    }
    if let Some(table) = error.table() {
        context = context.with(field::str("db.table", table.to_owned()));
    }

    let mut retry_after = None;
    let mut code_override = None;

    let code = error.code().map(|code| code.into_owned());
    if let Some(ref sqlstate) = code {
        context = context.with(field::str("db.code", sqlstate.clone()));
        if let Some((_, secs)) = SQLSTATE_RETRY_HINTS
            .iter()
            .find(|(state, _)| *state == sqlstate.as_str())
        {
            retry_after = Some(*secs);
        }
        if let Some((_, app_code)) = SQLSTATE_CODE_OVERRIDES
            .iter()
            .find(|(state, _)| *state == sqlstate.as_str())
        {
            code_override = Some(app_code.clone());
        }
    }

    let category = match error.kind() {
        SqlxErrorKind::UniqueViolation => AppErrorKind::Conflict,
        SqlxErrorKind::ForeignKeyViolation => AppErrorKind::Conflict,
        SqlxErrorKind::NotNullViolation | SqlxErrorKind::CheckViolation => {
            AppErrorKind::Validation
        }
        _ => AppErrorKind::Database
    };

    context = context.category(category);
    if let Some(code) = code_override {
        context = context.code(code);
    }

    (context, retry_after)
}

#[cfg(feature = "sqlx-migrate")]
fn build_migrate_context(err: &MigrateError) -> Context {
    if is_invalid_mix(err) {
        return Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "invalid_mix"));
    }

    match err {
        MigrateError::Execute(inner) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "execute"))
            .with(field::str("migration.source", inner.to_string())),
        MigrateError::ExecuteMigration(inner, version) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "execute_migration"))
            .with(field::i64("migration.version", *version))
            .with(field::str("migration.source", inner.to_string())),
        MigrateError::Source(source) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "source"))
            .with(field::str("migration.source", source.to_string())),
        MigrateError::VersionMissing(version) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "version_missing"))
            .with(field::i64("migration.version", *version)),
        MigrateError::VersionMismatch(version) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "version_mismatch"))
            .with(field::i64("migration.version", *version)),
        MigrateError::VersionNotPresent(version) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "version_not_present"))
            .with(field::i64("migration.version", *version)),
        MigrateError::VersionTooOld(version, latest) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "version_too_old"))
            .with(field::i64("migration.version", *version))
            .with(field::i64("migration.latest", *latest)),
        MigrateError::VersionTooNew(version, latest) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "version_too_new"))
            .with(field::i64("migration.version", *version))
            .with(field::i64("migration.latest", *latest)),
        MigrateError::ForceNotSupported => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "force_not_supported")),
        MigrateError::Dirty(version) => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "dirty"))
            .with(field::i64("migration.version", *version)),
        _ => Context::new(AppErrorKind::Database)
            .with(field::str("migration.phase", "unclassified"))
            .with(field::str("migration.detail", err.to_string()))
    }
}

#[cfg(feature = "sqlx-migrate")]
fn is_invalid_mix(err: &MigrateError) -> bool {
    #[allow(deprecated)]
    {
        matches!(err, MigrateError::InvalidMixReversibleAndSimple)
    }
}

#[cfg(all(test, feature = "sqlx"))]
mod tests_sqlx {
    use std::fmt;

    use super::*;
    use crate::{AppCode, AppErrorKind, FieldValue};

    #[test]
    fn row_not_found_maps_to_not_found() {
        let err: Error = SqlxError::RowNotFound.into();
        assert!(matches!(err.kind, AppErrorKind::NotFound));
    }

    #[test]
    fn io_error_maps_to_dependency_unavailable() {
        let io_err = std::io::Error::other("boom");
        let err: Error = SqlxError::Io(io_err).into();
        assert!(matches!(err.kind, AppErrorKind::DependencyUnavailable));
        let metadata = err.metadata();
        assert_eq!(
            metadata.get("db.reason"),
            Some(&FieldValue::Str("io_error".into()))
        );
    }

    #[test]
    fn unique_violation_sets_code_override() {
        let db_err = DummyDbError {
            message:    "duplicate key".into(),
            code:       Some("23505".into()),
            constraint: Some("users_email_key".into()),
            table:      Some("users".into()),
            kind:       SqlxErrorKind::UniqueViolation
        };
        let err: Error = SqlxError::Database(Box::new(db_err)).into();
        assert_eq!(err.kind, AppErrorKind::Conflict);
        assert_eq!(err.code, AppCode::UserAlreadyExists);
        let metadata = err.metadata();
        assert_eq!(
            metadata.get("db.constraint"),
            Some(&FieldValue::Str("users_email_key".into()))
        );
    }

    #[test]
    fn serialization_failure_carries_retry_hint() {
        let db_err = DummyDbError {
            message:    "serialization failure".into(),
            code:       Some("40001".into()),
            constraint: None,
            table:      None,
            kind:       SqlxErrorKind::Other
        };
        let err: Error = SqlxError::Database(Box::new(db_err)).into();
        assert_eq!(err.retry.map(|r| r.after_seconds), Some(1));
        assert_eq!(
            err.metadata().get("db.retry_after_hint_secs"),
            Some(&FieldValue::U64(1))
        );
    }

    #[derive(Debug)]
    struct DummyDbError {
        message:    String,
        code:       Option<String>,
        constraint: Option<String>,
        table:      Option<String>,
        kind:       SqlxErrorKind
    }

    impl fmt::Display for DummyDbError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.message)
        }
    }

    impl std::error::Error for DummyDbError {}

    impl DatabaseError for DummyDbError {
        fn message(&self) -> &str {
            &self.message
        }

        fn code(&self) -> Option<std::borrow::Cow<'_, str>> {
            self.code.as_deref().map(std::borrow::Cow::Borrowed)
        }

        fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
            self
        }

        fn constraint(&self) -> Option<&str> {
            self.constraint.as_deref()
        }

        fn table(&self) -> Option<&str> {
            self.table.as_deref()
        }

        fn kind(&self) -> SqlxErrorKind {
            match self.kind {
                SqlxErrorKind::UniqueViolation => SqlxErrorKind::UniqueViolation,
                SqlxErrorKind::ForeignKeyViolation => SqlxErrorKind::ForeignKeyViolation,
                SqlxErrorKind::NotNullViolation => SqlxErrorKind::NotNullViolation,
                SqlxErrorKind::CheckViolation => SqlxErrorKind::CheckViolation,
                SqlxErrorKind::Other => SqlxErrorKind::Other,
                _ => SqlxErrorKind::Other
            }
        }
    }
}
