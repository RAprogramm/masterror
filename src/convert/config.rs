// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Convert [`config::ConfigError`] into [`Error`],
//! producing [`AppErrorKind::Config`].
//!
//! Enabled with the `config` feature.
//!
//! ## Example
//!
//! ```rust,ignore
//! use config::ConfigError;
//! use masterror::{AppErrorKind, Error};
//!
//! let err = ConfigError::Message("missing key".into());
//! let app_err: Error = err.into();
//! assert!(matches!(app_err.kind, AppErrorKind::Config));
//! ```
#[cfg(feature = "config")]
use config::ConfigError;

#[cfg(feature = "config")]
use crate::{AppErrorKind, Context, Error, field};

#[cfg(feature = "config")]
#[cfg_attr(docsrs, doc(cfg(feature = "config")))]
impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        build_context(&err).into_error(err)
    }
}

#[cfg(feature = "config")]
fn build_context(error: &ConfigError) -> Context {
    match error {
        ConfigError::Frozen => {
            Context::new(AppErrorKind::Config).with(field::str("config.phase", "frozen"))
        }
        ConfigError::NotFound(key) => Context::new(AppErrorKind::Config)
            .with(field::str("config.phase", "not_found"))
            .with(field::str("config.key", key.clone())),
        ConfigError::PathParse {
            ..
        } => Context::new(AppErrorKind::Config).with(field::str("config.phase", "path_parse")),
        ConfigError::FileParse {
            uri, ..
        } => {
            let mut ctx =
                Context::new(AppErrorKind::Config).with(field::str("config.phase", "file_parse"));
            if let Some(path) = uri {
                ctx = ctx.with(field::str("config.uri", path.clone()));
            }
            ctx
        }
        ConfigError::Type {
            origin,
            unexpected,
            expected,
            key
        } => {
            let mut ctx = Context::new(AppErrorKind::Config)
                .with(field::str("config.phase", "type"))
                .with(field::str("config.expected", *expected))
                .with(field::str("config.unexpected", unexpected.to_string()));
            if let Some(origin) = origin {
                ctx = ctx.with(field::str("config.origin", origin.clone()));
            }
            if let Some(key) = key {
                ctx = ctx.with(field::str("config.key", key.clone()));
            }
            ctx
        }
        ConfigError::At {
            origin,
            key,
            ..
        } => {
            let mut ctx =
                Context::new(AppErrorKind::Config).with(field::str("config.phase", "at"));
            if let Some(origin) = origin {
                ctx = ctx.with(field::str("config.origin", origin.clone()));
            }
            if let Some(key) = key {
                ctx = ctx.with(field::str("config.key", key.clone()));
            }
            ctx
        }
        ConfigError::Message(message) => Context::new(AppErrorKind::Config)
            .with(field::str("config.phase", "message"))
            .with(field::str("config.message", message.clone())),
        ConfigError::Foreign(_) => {
            Context::new(AppErrorKind::Config).with(field::str("config.phase", "foreign"))
        }
        other => Context::new(AppErrorKind::Config)
            .with(field::str("config.phase", "unclassified"))
            .with(field::str("config.debug", other.to_string()))
    }
}

#[cfg(all(test, feature = "config"))]
mod tests {
    use config::ConfigError;

    use super::*;
    use crate::{AppErrorKind, FieldValue};

    #[test]
    fn maps_to_config_kind() {
        let err = ConfigError::Message("dummy".into());
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("config.phase"),
            Some(&FieldValue::Str("message".into()))
        );
    }

    #[test]
    fn frozen_error_maps_correctly() {
        let err = ConfigError::Frozen;
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        assert_eq!(
            app_err.metadata().get("config.phase"),
            Some(&FieldValue::Str("frozen".into()))
        );
    }

    #[test]
    fn not_found_error_captures_key() {
        let err = ConfigError::NotFound("database.url".into());
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("config.phase"),
            Some(&FieldValue::Str("not_found".into()))
        );
        assert_eq!(
            metadata.get("config.key"),
            Some(&FieldValue::Str("database.url".into()))
        );
    }

    #[test]
    fn path_parse_error_maps_correctly() {
        let err = ConfigError::PathParse {
            cause: Box::new(std::io::Error::other("invalid path"))
        };
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        assert_eq!(
            app_err.metadata().get("config.phase"),
            Some(&FieldValue::Str("path_parse".into()))
        );
    }

    #[test]
    fn file_parse_error_without_uri() {
        let err = ConfigError::FileParse {
            uri:   None,
            cause: Box::new(std::io::Error::other("disk"))
        };
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        assert_eq!(
            app_err.metadata().get("config.phase"),
            Some(&FieldValue::Str("file_parse".into()))
        );
        assert!(app_err.metadata().get("config.uri").is_none());
    }

    #[test]
    fn file_parse_error_with_uri() {
        let err = ConfigError::FileParse {
            uri:   Some("/etc/app/config.toml".into()),
            cause: Box::new(std::io::Error::other("disk"))
        };
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("config.phase"),
            Some(&FieldValue::Str("file_parse".into()))
        );
        assert_eq!(
            metadata.get("config.uri"),
            Some(&FieldValue::Str("/etc/app/config.toml".into()))
        );
    }

    #[test]
    fn at_error_with_all_fields() {
        let err = ConfigError::At {
            origin: Some("env.toml".into()),
            key:    Some("database.host".into()),
            error:  Box::new(ConfigError::Message("invalid".into()))
        };
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("config.phase"),
            Some(&FieldValue::Str("at".into()))
        );
        assert_eq!(
            metadata.get("config.origin"),
            Some(&FieldValue::Str("env.toml".into()))
        );
        assert_eq!(
            metadata.get("config.key"),
            Some(&FieldValue::Str("database.host".into()))
        );
    }

    #[test]
    fn at_error_without_optional_fields() {
        let err = ConfigError::At {
            origin: None,
            key:    None,
            error:  Box::new(ConfigError::Message("error".into()))
        };
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("config.phase"),
            Some(&FieldValue::Str("at".into()))
        );
        assert!(metadata.get("config.origin").is_none());
        assert!(metadata.get("config.key").is_none());
    }

    #[test]
    fn message_error_preserves_message() {
        let err = ConfigError::Message("custom error message".into());
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("config.phase"),
            Some(&FieldValue::Str("message".into()))
        );
        assert_eq!(
            metadata.get("config.message"),
            Some(&FieldValue::Str("custom error message".into()))
        );
    }

    #[test]
    fn foreign_error_maps_correctly() {
        let foreign_err = Box::new(std::io::Error::other("external"))
            as Box<dyn std::error::Error + Send + Sync>;
        let err = ConfigError::Foreign(foreign_err);
        let app_err = Error::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
        assert_eq!(
            app_err.metadata().get("config.phase"),
            Some(&FieldValue::Str("foreign".into()))
        );
    }

    #[test]
    fn error_preserves_source() {
        let err = ConfigError::Message("source test".into());
        let app_err = Error::from(err);
        assert!(app_err.source_ref().is_some());
    }
}
