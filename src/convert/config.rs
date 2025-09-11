//! Convert [`config::ConfigError`] into [`AppError`],
//! producing [`AppErrorKind::Config`].
//!
//! Enabled with the `config` feature.
//!
//! ## Example
//!
//! ```rust,ignore
//! use config::ConfigError;
//! use masterror::{AppError, AppErrorKind};
//!
//! let err = ConfigError::Message("missing key".into());
//! let app_err: AppError = err.into();
//! assert!(matches!(app_err.kind, AppErrorKind::Config));
//! ```
#[cfg(feature = "config")]
use config::ConfigError;

#[cfg(feature = "config")]
use crate::AppError;

#[cfg(feature = "config")]
#[cfg_attr(docsrs, doc(cfg(feature = "config")))]
impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError::config(err.to_string())
    }
}

#[cfg(all(test, feature = "config"))]
mod tests {
    use config::ConfigError;

    use crate::{AppError, AppErrorKind};

    #[test]
    fn maps_to_config_kind() {
        let err = ConfigError::Message("dummy".into());
        let app_err = AppError::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
    }
}
