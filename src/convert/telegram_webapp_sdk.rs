//! Conversion from
//! [`telegram_webapp_sdk::utils::validate_init_data::ValidationError`] into
//! [`AppError`].
//!
//! Enabled with the `telegram-webapp-sdk` feature flag.
//!
//! ## Mapping
//!
//! All [`ValidationError`] variants are mapped to `AppErrorKind::TelegramAuth`
//! and the original error text is preserved in the message.
//!
//! ## Rationale
//!
//! Failing to validate Telegram `initData` indicates an authentication problem
//! with the incoming request. Mapping to `TelegramAuth` keeps this distinction
//! explicit and allows callers to handle it separately from generic bad
//! requests.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppError, AppErrorKind};
//! use telegram_webapp_sdk::utils::validate_init_data::ValidationError;
//!
//! fn convert(err: ValidationError) -> AppError {
//!     err.into()
//! }
//!
//! let e = convert(ValidationError::SignatureMismatch);
//! assert!(matches!(e.kind, AppErrorKind::TelegramAuth));
//! ```

#[cfg(feature = "telegram-webapp-sdk")]
use telegram_webapp_sdk::utils::validate_init_data::ValidationError;

#[cfg(feature = "telegram-webapp-sdk")]
use crate::AppError;

/// Map [`ValidationError`] into an [`AppError`] with kind `TelegramAuth`.
#[cfg(feature = "telegram-webapp-sdk")]
#[cfg_attr(docsrs, doc(cfg(feature = "telegram-webapp-sdk")))]
impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::telegram_auth(err.to_string())
    }
}

#[cfg(all(test, feature = "telegram-webapp-sdk"))]
mod tests {
    use telegram_webapp_sdk::utils::validate_init_data::ValidationError;

    use super::*;
    use crate::AppErrorKind;

    #[test]
    fn validation_error_maps_to_telegram_auth() {
        let err: AppError = ValidationError::SignatureMismatch.into();
        assert!(matches!(err.kind, AppErrorKind::TelegramAuth));
    }
}
