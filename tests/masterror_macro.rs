#![allow(non_shorthand_field_patterns)]

// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::{error::Error as StdError, sync::Arc};

use masterror::{
    AppCode, AppErrorKind, Error as MasterrorError, FieldRedaction, Masterror, MessageEditPolicy,
    mapping::{GrpcMapping, HttpMapping, ProblemMapping}
};

#[derive(Debug, Masterror)]
#[error("missing feature flag {flag}")]
#[masterror(
    code = AppCode::NotFound,
    category = AppErrorKind::NotFound,
    message,
    redact(message, fields("user_id" = hash)),
    telemetry(
        Some(masterror::field::str("user_id", user_id.clone())),
        attempt.map(|value| masterror::field::u64("attempt", value))
    ),
    map.grpc = 5,
    map.problem = "https://errors.example.com/not-found"
)]
struct MissingFlag {
    user_id: String,
    flag:    &'static str,
    attempt: Option<u64>,
    #[source]
    source:  Option<std::io::Error>
}

#[derive(Debug, Masterror)]
enum ApiError {
    #[error("invalid payload: {details}")]
    #[masterror(
        code = AppCode::BadRequest,
        category = AppErrorKind::BadRequest,
        message,
        telemetry(Some(masterror::field::str("details", details))),
        map.problem = "https://errors.example.com/bad-request"
    )]
    BadPayload {
        details: &'static str,
        #[allow(non_shorthand_field_patterns)]
        #[source]
        _source: std::io::Error
    },
    #[error("storage offline")]
    #[masterror(
        code = AppCode::Service,
        category = AppErrorKind::Service,
        telemetry(),
        map.grpc = 14
    )]
    StorageOffline
}

#[test]
fn struct_masterror_conversion_populates_metadata_and_source() {
    let source = std::io::Error::other("backend down");
    let err = MissingFlag {
        user_id: "alice".into(),
        flag:    "beta",
        attempt: Some(3),
        source:  Some(source)
    };
    let converted: MasterrorError = err.into();
    assert_eq!(converted.code, AppCode::NotFound);
    assert_eq!(converted.kind, AppErrorKind::NotFound);
    assert_eq!(converted.edit_policy, MessageEditPolicy::Redact);
    assert!(
        converted
            .message
            .as_deref()
            .is_some_and(|message| message.contains("beta"))
    );
    let user_id = converted
        .metadata()
        .get("user_id")
        .and_then(|value| match value {
            masterror::FieldValue::Str(value) => Some(value.as_ref()),
            _ => None
        });
    assert_eq!(user_id, Some("alice"));
    let attempt = converted
        .metadata()
        .get("attempt")
        .and_then(|value| match value {
            masterror::FieldValue::U64(value) => Some(*value),
            _ => None
        });
    assert_eq!(attempt, Some(3));
    assert!(converted.source_ref().is_some());
    let converted_source = StdError::source(&converted).expect("masterror source");
    assert!(converted_source.is::<std::io::Error>());
    assert_eq!(
        MissingFlag::HTTP_MAPPING,
        HttpMapping::new(AppCode::NotFound, AppErrorKind::NotFound)
    );
    assert_eq!(
        converted.metadata().redaction("user_id"),
        Some(FieldRedaction::Hash)
    );
    assert_eq!(MissingFlag::HTTP_MAPPING.status(), 404);
    let grpc = MissingFlag::GRPC_MAPPING.expect("grpc mapping");
    assert_eq!(grpc.status(), 5);
    assert_eq!(grpc.kind(), AppErrorKind::NotFound);
    let problem = MissingFlag::PROBLEM_MAPPING.expect("problem mapping");
    assert_eq!(problem.type_uri(), "https://errors.example.com/not-found");
}

#[test]
fn enum_masterror_conversion_handles_variants() {
    let io_error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "format");
    let payload = ApiError::BadPayload {
        details: "missing field",
        _source: io_error
    };
    let converted: MasterrorError = payload.into();
    assert_eq!(converted.code, AppCode::BadRequest);
    assert_eq!(converted.kind, AppErrorKind::BadRequest);
    assert!(converted.metadata().get("details").is_some_and(
        |value| matches!(value, masterror::FieldValue::Str(detail) if detail == "missing field")
    ));
    assert!(converted.source_ref().is_some());
    let offline: MasterrorError = ApiError::StorageOffline.into();
    assert_eq!(offline.code, AppCode::Service);
    assert_eq!(offline.kind, AppErrorKind::Service);
    assert!(offline.metadata().is_empty());
    assert_eq!(ApiError::HTTP_MAPPINGS.len(), 2);
    assert!(
        ApiError::HTTP_MAPPINGS
            .iter()
            .any(|mapping| mapping.kind() == AppErrorKind::BadRequest)
    );
    assert_eq!(
        ApiError::GRPC_MAPPINGS,
        &[GrpcMapping::new(
            AppCode::Service,
            AppErrorKind::Service,
            14
        )]
    );
    assert_eq!(
        ApiError::PROBLEM_MAPPINGS,
        &[ProblemMapping::new(
            AppCode::BadRequest,
            AppErrorKind::BadRequest,
            "https://errors.example.com/bad-request"
        )]
    );
}

#[test]
fn masterror_preserves_arc_source_without_extra_clone() {
    let source = Arc::new(ArcLeafError);
    let converted: MasterrorError = ArcSourceError {
        source: source.clone()
    }
    .into();
    assert_eq!(Arc::strong_count(&source), 2);
    let stored = converted
        .source_ref()
        .and_then(|src| src.downcast_ref::<ArcLeafError>())
        .expect("arc source");
    assert!(std::ptr::eq(stored, &*source));
}
#[derive(Debug)]
struct ArcLeafError;

impl std::fmt::Display for ArcLeafError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("arc leaf")
    }
}

impl std::error::Error for ArcLeafError {}

#[derive(Debug, Masterror)]
#[error("arc leaf source")]
#[masterror(
    code = AppCode::Internal,
    category = AppErrorKind::Internal,
    message
)]
struct ArcSourceError {
    #[source]
    source: Arc<ArcLeafError>
}
