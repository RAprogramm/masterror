//! Tonic integration: convert [`crate::Error`] into [`tonic::Status`].
//!
//! Enabled with the `tonic` feature flag.
//!
//! ## Behavior
//! - Maps [`AppCode`] to the corresponding gRPC [`tonic::Code`].
//! - Emits retry/authentication hints via metadata when available.
//! - Propagates public metadata only when the error is not marked as
//!   redactable.
//! - Redacts the message automatically when the error is private.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::AppError;
//!
//! let status = tonic::Status::from(AppError::not_found("missing"));
//! assert_eq!(status.code(), tonic::Code::NotFound);
//! ```

use core::convert::Infallible;
use std::borrow::Cow;

use tonic::{
    Code, Status,
    metadata::{MetadataMap, MetadataValue}
};

#[cfg(test)]
use crate::CODE_MAPPINGS;
use crate::{
    AppErrorKind, Error, FieldRedaction, FieldValue, MessageEditPolicy, Metadata, RetryAdvice,
    app_error::duration_to_string, mapping_for_code
};

/// Error alias retained for backwards compatibility with 0.20 conversions.
///
/// Since Rust 1.90 the standard library implements [`TryFrom`] for every
/// [`Into`] conversion with [`core::convert::Infallible`] as the error type.
/// Tonic conversions are therefore guaranteed to succeed, and this alias keeps
/// the historic [`StatusConversionError`] name available for downstream APIs.
///
/// # Examples
/// ```rust,ignore
/// use masterror::{AppError, StatusConversionError};
/// use tonic::{Code, Status};
///
/// let status: Result<Status, StatusConversionError> = Status::try_from(
///     AppError::not_found("missing")
/// );
/// let status = status.expect("conversion cannot fail");
/// assert_eq!(status.code(), Code::NotFound);
/// ```
pub type StatusConversionError = Infallible;

impl From<Error> for Status {
    fn from(error: Error) -> Self {
        status_from_error(&error)
    }
}

fn status_from_error(error: &Error) -> Status {
    error.emit_telemetry();

    let mapping = mapping_for_code(&error.code);
    let grpc_code = Code::from_i32(mapping.grpc().value);
    let detail = sanitize_detail(error.message.as_ref(), error.kind, error.edit_policy);
    let mut meta = MetadataMap::new();

    insert_ascii(&mut meta, "app-code", error.code.as_str());
    insert_ascii(
        &mut meta,
        "app-http-status",
        mapping.http_status().to_string()
    );
    insert_ascii(&mut meta, "app-problem-type", mapping.problem_type());

    if let Some(advice) = error.retry {
        insert_retry(&mut meta, advice);
    }
    if let Some(challenge) = error.www_authenticate.as_deref()
        && is_ascii_metadata_value(challenge)
    {
        insert_ascii(&mut meta, "www-authenticate", challenge);
    }

    if !matches!(error.edit_policy, MessageEditPolicy::Redact) {
        attach_metadata(&mut meta, error.metadata());
    }

    Status::with_metadata(grpc_code, detail, meta)
}

fn sanitize_detail(
    message: Option<&Cow<'static, str>>,
    kind: AppErrorKind,
    policy: MessageEditPolicy
) -> String {
    if matches!(policy, MessageEditPolicy::Redact) {
        return kind.to_string();
    }

    message.map_or_else(|| kind.to_string(), |msg| msg.as_ref().to_owned())
}

fn insert_retry(meta: &mut MetadataMap, retry: RetryAdvice) {
    insert_ascii(meta, "retry-after", retry.after_seconds.to_string());
}

fn attach_metadata(meta: &mut MetadataMap, metadata: &Metadata) {
    for (name, value, redaction) in metadata.iter_with_redaction() {
        if !matches!(redaction, FieldRedaction::None) {
            continue;
        }
        if !is_safe_metadata_key(name) {
            continue;
        }
        if let Some(serialized) = metadata_value_to_ascii(value) {
            insert_ascii(meta, name, serialized);
        }
    }
}

fn insert_ascii(meta: &mut MetadataMap, key: &'static str, value: impl AsRef<str>) {
    if !is_safe_metadata_key(key) {
        return;
    }
    let value = value.as_ref();
    if !is_ascii_metadata_value(value) {
        return;
    }
    if let Ok(metadata_value) = MetadataValue::try_from(value) {
        let _ = meta.insert(key, metadata_value);
    }
}

fn metadata_value_to_ascii(value: &FieldValue) -> Option<Cow<'_, str>> {
    match value {
        FieldValue::Str(value) => {
            let text = value.as_ref();
            is_ascii_metadata_value(text).then_some(Cow::Borrowed(text))
        }
        FieldValue::I64(value) => Some(Cow::Owned(value.to_string())),
        FieldValue::U64(value) => Some(Cow::Owned(value.to_string())),
        FieldValue::F64(value) => Some(Cow::Owned(value.to_string())),
        FieldValue::Bool(value) => Some(Cow::Borrowed(if *value { "true" } else { "false" })),
        FieldValue::Uuid(value) => Some(Cow::Owned(value.to_string())),
        FieldValue::Duration(value) => Some(Cow::Owned(duration_to_string(*value))),
        FieldValue::Ip(value) => Some(Cow::Owned(value.to_string())),
        #[cfg(feature = "serde_json")]
        FieldValue::Json(_) => None
    }
}

fn is_safe_metadata_key(key: &str) -> bool {
    !key.is_empty()
        && key
            .bytes()
            .all(|ch| matches!(ch, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.'))
}

fn is_ascii_metadata_value(value: &str) -> bool {
    value.bytes().all(|ch| matches!(ch, 0x20..=0x7E))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AppError, AppErrorKind, field};

    #[test]
    fn status_maps_codes_correctly() {
        for (code, mapping) in CODE_MAPPINGS.iter() {
            let err = AppError::with(mapping.kind(), format!("{:?}", code));
            let status = Status::from(err);
            assert_eq!(status.code(), Code::from_i32(mapping.grpc().value));
            let expected_detail = format!("{:?}", code);
            assert_eq!(
                status.message(),
                expected_detail,
                "unexpected message for {:?}",
                code
            );
        }
    }

    #[test]
    fn redacted_errors_hide_metadata() {
        let err = AppError::internal("secret")
            .redactable()
            .with_field(field::str("request_id", "abc"));
        let status = Status::from(err);
        assert_eq!(status.message(), AppErrorKind::Internal.to_string());
        assert!(status.metadata().get("request_id").is_none());
    }

    #[test]
    fn public_metadata_is_propagated() {
        let err = AppError::service("downstream")
            .with_field(field::str("request_id", "abc"))
            .with_field(field::u64("attempt", 2));
        let status = Status::from(err);
        assert_eq!(
            status
                .metadata()
                .get("request_id")
                .and_then(|value| value.to_str().ok()),
            Some("abc")
        );
        assert_eq!(
            status
                .metadata()
                .get("attempt")
                .and_then(|value| value.to_str().ok()),
            Some("2")
        );
    }
}
