// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion from
//! [`telegram_webapp_sdk::utils::validate_init_data::ValidationError`] into
//! [`Error`].
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
//! '''rust
//! # #[cfg(feature = "telegram-webapp-sdk")]
//! # {
//! '''rust,ignore
//! use masterror::{AppErrorKind, Error};
//! use telegram_webapp_sdk::utils::validate_init_data::ValidationError;
//!
//! fn convert(err: ValidationError) -> Error {
//!     err.into()
//! }
//!
//! let e = convert(ValidationError::SignatureMismatch);
//! assert!(matches!(e.kind, AppErrorKind::TelegramAuth));
//! assert_eq!(e.message.as_deref(), Some("signature mismatch"));
//! # }
//! '''

#[cfg(feature = "telegram-webapp-sdk")]
use telegram_webapp_sdk::utils::validate_init_data::ValidationError;

#[cfg(feature = "telegram-webapp-sdk")]
use crate::{AppErrorKind, Context, Error, field};

/// Map [`ValidationError`] into an [`crate::AppError`] with kind
/// `TelegramAuth`.
#[cfg(feature = "telegram-webapp-sdk")]
#[cfg_attr(docsrs, doc(cfg(feature = "telegram-webapp-sdk")))]
impl From<ValidationError> for Error {
    fn from(err: ValidationError) -> Self {
        build_context(&err).into_error(err)
    }
}

#[cfg(feature = "telegram-webapp-sdk")]
fn build_context(error: &ValidationError) -> Context {
    match error {
        ValidationError::MissingField(field) => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_webapp.reason", "missing_field"))
            .with(field::str("telegram_webapp.field", (*field).to_owned())),
        ValidationError::InvalidEncoding => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_webapp.reason", "invalid_encoding")),
        ValidationError::InvalidSignatureEncoding => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str(
                "telegram_webapp.reason",
                "invalid_signature_encoding"
            )),
        ValidationError::SignatureMismatch => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_webapp.reason", "signature_mismatch")),
        ValidationError::InvalidPublicKey => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_webapp.reason", "invalid_public_key"))
    }
}

#[cfg(all(test, feature = "telegram-webapp-sdk"))]
mod tests {
    use telegram_webapp_sdk::utils::validate_init_data::ValidationError;

    use super::*;
    use crate::{AppErrorKind, FieldValue};

    #[test]
    fn all_variants_map_to_telegram_auth_and_preserve_message() {
        let cases = vec![
            ValidationError::MissingField("hash"),
            ValidationError::InvalidEncoding,
            ValidationError::InvalidSignatureEncoding,
            ValidationError::SignatureMismatch,
            ValidationError::InvalidPublicKey,
        ];

        for case in cases {
            let app: Error = case.into();
            assert!(matches!(app.kind, AppErrorKind::TelegramAuth));
            assert!(app.metadata().get("telegram_webapp.reason").is_some());
        }
    }

    #[test]
    fn validation_error_maps_to_telegram_auth() {
        let err: Error = ValidationError::SignatureMismatch.into();
        assert!(matches!(err.kind, AppErrorKind::TelegramAuth));
        assert_eq!(
            err.metadata().get("telegram_webapp.reason"),
            Some(&FieldValue::Str("signature_mismatch".into()))
        );
    }
}
