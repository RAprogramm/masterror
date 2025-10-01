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
}
