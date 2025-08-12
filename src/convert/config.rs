#[cfg(feature = "config")]
use config::ConfigError;

#[cfg(feature = "config")]
use crate::AppError;

#[cfg(feature = "config")]
impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError::config(err.to_string())
    }
}
