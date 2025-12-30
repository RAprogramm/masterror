// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion from [`init_data_rs::InitDataError`] into [`Error`].
//!
//! Enabled with the `init-data` feature flag.
//!
//! ## Mapping
//!
//! All [`InitDataError`] variants are mapped to `AppErrorKind::TelegramAuth`
//! and the original error text is preserved in the message.
//!
//! ## Rationale
//!
//! Failing to validate Telegram Mini Apps `initData` indicates an
//! authentication problem with the incoming request. Mapping to `TelegramAuth`
//! keeps this distinction explicit and allows callers to handle it separately
//! from generic bad requests.
//!
//! ## Example
//!
//! '''rust
//! # #[cfg(feature = "init-data")]
//! # {
//! '''rust,ignore
//! use init_data_rs::InitDataError;
//! use masterror::{AppErrorKind, Error};
//!
//! fn convert(err: InitDataError) -> Error {
//!     err.into()
//! }
//!
//! let e = convert(InitDataError::HashMissing);
//! assert!(matches!(e.kind, AppErrorKind::TelegramAuth));
//! # }
//! '''

#[cfg(feature = "init-data")]
use init_data_rs::InitDataError;

#[cfg(feature = "init-data")]
use crate::{AppErrorKind, Context, Error, field};

#[cfg(feature = "init-data")]
#[cfg_attr(docsrs, doc(cfg(feature = "init-data")))]
impl From<InitDataError> for Error {
    fn from(err: InitDataError) -> Self {
        build_context(&err).into_error(err)
    }
}

#[cfg(feature = "init-data")]
fn build_context(error: &InitDataError) -> Context {
    match error {
        InitDataError::AuthDateMissing => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "auth_date_missing")),
        InitDataError::HashMissing => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "hash_missing")),
        InitDataError::HashInvalid => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "hash_invalid")),
        InitDataError::UnexpectedFormat(details) => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "unexpected_format"))
            .with(field::str("telegram_init_data.details", details.clone())),
        InitDataError::Expired => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "expired")),
        InitDataError::Internal(details) => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "internal"))
            .with(field::str("telegram_init_data.details", details.clone())),
        InitDataError::SignatureMissing => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "signature_missing")),
        InitDataError::SignatureInvalid(details) => Context::new(AppErrorKind::TelegramAuth)
            .with(field::str("telegram_init_data.reason", "signature_invalid"))
            .with(field::str("telegram_init_data.details", details.clone()))
    }
}

#[cfg(all(test, feature = "init-data"))]
mod tests {
    use init_data_rs::InitDataError;

    use super::*;
    use crate::{AppErrorKind, FieldValue};

    #[test]
    fn all_variants_map_to_telegram_auth() {
        let cases: Vec<InitDataError> = vec![
            InitDataError::AuthDateMissing,
            InitDataError::HashMissing,
            InitDataError::HashInvalid,
            InitDataError::UnexpectedFormat("bad format".into()),
            InitDataError::Expired,
            InitDataError::Internal("oops".into()),
            InitDataError::SignatureMissing,
            InitDataError::SignatureInvalid("bad sig".into()),
        ];
        for case in cases {
            let app: Error = case.into();
            assert!(matches!(app.kind, AppErrorKind::TelegramAuth));
            assert!(app.metadata().get("telegram_init_data.reason").is_some());
        }
    }

    #[test]
    fn hash_missing_maps_correctly() {
        let err: Error = InitDataError::HashMissing.into();
        assert!(matches!(err.kind, AppErrorKind::TelegramAuth));
        assert_eq!(
            err.metadata().get("telegram_init_data.reason"),
            Some(&FieldValue::Str("hash_missing".into()))
        );
    }

    #[test]
    fn expired_maps_correctly() {
        let err: Error = InitDataError::Expired.into();
        assert!(matches!(err.kind, AppErrorKind::TelegramAuth));
        assert_eq!(
            err.metadata().get("telegram_init_data.reason"),
            Some(&FieldValue::Str("expired".into()))
        );
    }

    #[test]
    fn unexpected_format_preserves_details() {
        let err: Error = InitDataError::UnexpectedFormat("missing field".into()).into();
        assert!(matches!(err.kind, AppErrorKind::TelegramAuth));
        assert_eq!(
            err.metadata().get("telegram_init_data.reason"),
            Some(&FieldValue::Str("unexpected_format".into()))
        );
        assert_eq!(
            err.metadata().get("telegram_init_data.details"),
            Some(&FieldValue::Str("missing field".into()))
        );
    }

    #[test]
    fn signature_invalid_preserves_details() {
        let err: Error = InitDataError::SignatureInvalid("mismatch".into()).into();
        assert!(matches!(err.kind, AppErrorKind::TelegramAuth));
        assert_eq!(
            err.metadata().get("telegram_init_data.reason"),
            Some(&FieldValue::Str("signature_invalid".into()))
        );
        assert_eq!(
            err.metadata().get("telegram_init_data.details"),
            Some(&FieldValue::Str("mismatch".into()))
        );
    }
}
