// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion from [`validator::ValidationErrors`] into [`Error`].
//!
//! Enabled with the `validator` feature flag.
//!
//! ## Mapping
//!
//! All validation failures are mapped to `AppErrorKind::Validation` with
//! the stringified [`ValidationErrors`] content in `message`.
//!
//! ## Rationale
//!
//! [`validator::ValidationErrors`] provides structured error details, but
//! serializing them directly into the public API payload is not always desired.
//! Here we convert them to a human-readable string for logs and simple clients.
//! If you need to expose structured validation errors in JSON, extend your
//! `AppError` type to carry `serde_json::Value` and adjust this mapping
//! accordingly.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppErrorKind, AppResult, Error};
//! use validator::{Validate, ValidationError};
//!
//! #[derive(Validate)]
//! struct Payload {
//!     #[validate(length(min = 5))]
//!     name: String,
//! }
//!
//! fn validate_payload(p: Payload) -> AppResult<()> {
//!     p.validate()?;
//!     Ok(())
//! }
//!
//! let bad = Payload { name: "abc".into() };
//! let err = validate_payload(bad).unwrap_err();
//! assert!(matches!(err.kind, AppErrorKind::Validation));
//! ```

#[cfg(feature = "validator")]
use validator::{ValidationErrors, ValidationErrorsKind};

#[cfg(feature = "validator")]
use crate::{AppErrorKind, Context, Error, field};

/// Map [`validator::ValidationErrors`] into an [`crate::AppError`] with kind
/// `Validation`.
///
/// By default, the error is converted to a string for human-readable logs.
/// Consider extending `AppError` if you want to expose structured details.
#[cfg(feature = "validator")]
#[cfg_attr(docsrs, doc(cfg(feature = "validator")))]
impl From<ValidationErrors> for Error {
    fn from(err: ValidationErrors) -> Self {
        build_context(&err).into_error(err)
    }
}

#[cfg(feature = "validator")]
fn build_context(errors: &ValidationErrors) -> Context {
    let mut context = Context::new(AppErrorKind::Validation);

    let field_errors = errors.field_errors();
    if !field_errors.is_empty() {
        context = context.with(field::u64(
            "validation.field_count",
            field_errors.len() as u64
        ));

        let total: u64 = field_errors.values().map(|errs| errs.len() as u64).sum();
        if total > 0 {
            context = context.with(field::u64("validation.error_count", total));
        }

        let mut names = String::new();
        for (idx, name) in field_errors.keys().take(3).enumerate() {
            if idx > 0 {
                names.push(',');
            }
            names.push_str(name.as_ref());
        }
        if !names.is_empty() {
            context = context.with(field::str("validation.fields", names));
        }

        let mut codes: Vec<String> = Vec::new();
        for errors in field_errors.values() {
            for error in *errors {
                let code = error.code.as_ref();
                if codes.len() >= 3 {
                    break;
                }
                if codes.iter().any(|existing| existing == code) {
                    continue;
                }
                codes.push(code.to_string());
            }
        }
        if !codes.is_empty() {
            context = context.with(field::str("validation.codes", codes.join(",")));
        }
    }

    let has_nested = errors
        .errors()
        .values()
        .any(|kind| !matches!(kind, ValidationErrorsKind::Field(_)));
    if has_nested {
        context = context.with(field::bool("validation.has_nested", true));
    }

    context
}

#[cfg(all(test, feature = "validator"))]
mod tests {
    use validator::Validate;

    use super::*;
    use crate::{AppErrorKind, FieldValue};

    #[derive(Validate)]
    struct Payload {
        #[validate(range(min = 1))]
        val: i32
    }

    #[test]
    fn validation_errors_map_to_validation_kind() {
        let bad = Payload {
            val: 0
        };
        let err: Error = bad.validate().unwrap_err().into();
        assert!(matches!(err.kind, AppErrorKind::Validation));
        let metadata = err.metadata();
        assert_eq!(
            metadata.get("validation.field_count"),
            Some(&FieldValue::U64(1))
        );
    }
}
