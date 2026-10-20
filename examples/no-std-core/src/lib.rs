// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! `no_std` usage of the `masterror` core.
//!
//! The crate depends on `masterror` without the default `std` feature, so it
//! only relies on `core` and `alloc`. The very same sources build for
//! bare-metal targets — the CI checks that with
//! `cargo check -p no-std-core --target thumbv7em-none-eabihf`.
//!
//! The module covers the whole public surface of the crate in this
//! configuration:
//!
//! - both derive macros ([`derive@Error`] and [`derive@Masterror`]),
//! - the error taxonomy ([`AppErrorKind`]) and the stable codes ([`AppCode`]),
//! - the error type itself with its builders, context, metadata and
//!   introspection ([`AppError`], [`Context`], [`Metadata`], [`field`]),
//! - the wire payloads ([`ErrorResponse`] and [`ProblemJson`]) with their
//!   numeric status field,
//! - the protocol mappings and the template helpers ([`masterror::mapping`],
//!   [`masterror::error::template`]).
//!
//! Not available: the `http` interop. The `http` crate requires the standard
//! library, so the helpers returning `http::StatusCode` values stay behind the
//! `std` feature. The numeric status remains available through
//! [`AppErrorKind::http_status`].

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use masterror::{
    AppCode, AppError, AppErrorKind, AppResult, Context, DisplayMode, Error, ErrorResponse,
    Masterror, MessageEditPolicy, Metadata, ProblemJson, ResultExt, RetryAdvice,
    error::template::{ErrorTemplate, TemplateSegment},
    field, mapping_for_code
};

/// Domain error reported by a sensor reading.
///
/// The `masterror` derive works without the standard library: it implements
/// [`core::error::Error`] and converts the error into an [`AppError`].
#[derive(Debug, Error)]
#[error("sensor {sensor} reported {value}")]
#[app_error(kind = AppErrorKind::Validation, code = AppCode::Validation, message)]
pub struct SensorOutOfRange {
    /// Identifier of the sensor that produced the reading.
    pub sensor: &'static str,
    /// Value that was rejected.
    pub value:  i32
}

/// Domain error carrying its own code and category.
///
/// The second derive macro of the crate follows the same rules: it generates
/// the [`core::error::Error`] implementation and the conversion into
/// [`AppError`] without touching the standard library.
#[derive(Debug, Masterror)]
#[error("link {link} is unreachable")]
#[masterror(code = AppCode::Network, category = AppErrorKind::Internal, message)]
pub struct LinkUnreachable {
    /// Address of the unreachable link.
    pub link: &'static str
}

/// Converts a domain error into an [`AppError`] with the taxonomy attached.
#[must_use]
pub fn classify(reading: SensorOutOfRange) -> AppError {
    AppError::from(reading)
}

/// Converts a link error into an [`AppError`] with the taxonomy attached.
#[must_use]
pub fn classify_link(failure: LinkUnreachable) -> AppError {
    AppError::from(failure)
}

/// Builds an error with a message, a stable code and telemetry metadata.
///
/// Exercises the builder API of [`AppError`] without the standard library.
#[must_use]
pub fn build_error() -> AppError {
    AppError::with(AppErrorKind::Conflict, "the resource is locked")
        .with_code(AppCode::Conflict)
        .with_retry_after_secs(30)
        .with_www_authenticate("Bearer realm=\"sensors\"")
        .with_metadata(Metadata::from_fields([field::u64("queue_depth", 12)]))
        .with_field(field::str("phase", "acquire"))
        .with_fields([field::i64("attempts", 3), field::bool("retryable", true)])
        .redact_field("token", masterror::FieldRedaction::Redact)
}

/// Attaches a source error to an [`AppError`] and wraps it in a context.
#[must_use]
pub fn attach_source(error: AppError) -> AppError {
    error.with_context(SourceFailure)
}

/// Builds a context for a failing operation.
///
/// Exercises the [`Context`] builder: the code, the category, the metadata
/// fields, the redaction policy and the caller location.
#[must_use]
pub fn build_context() -> Context {
    let mut context = Context::new(AppErrorKind::Service)
        .code(AppCode::DependencyUnavailable)
        .category(AppErrorKind::Service)
        .with(field::str("phase", "publish"))
        .redact_field("token", masterror::FieldRedaction::Hash)
        .redact(true)
        .track_caller();
    let _ = context.redact_field_mut("token", masterror::FieldRedaction::Last4);
    context
}

/// Source error used to exercise the context and source APIs.
#[derive(Debug)]
pub struct SourceFailure;

impl core::fmt::Display for SourceFailure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("the sensor bus is offline")
    }
}

impl core::error::Error for SourceFailure {}

/// Wraps a failing result with a context message.
pub fn wrap_result() -> AppResult<u8> {
    let failure: Result<u8, SourceFailure> = Err(SourceFailure);
    failure.context("the reading could not be taken")
}

/// Wraps a failing result with the context built by [`build_context`].
pub fn wrap_with_context() -> AppResult<u8> {
    let failure: Result<u8, SourceFailure> = Err(SourceFailure);
    failure.ctx(build_context)
}

/// Renders the error as a wire payload without the standard library.
///
/// The status comes from the taxonomy, so the payload stays consistent with
/// the [`ProblemJson`] representation.
///
/// # Panics
///
/// Panics if the status of the error kind is not a valid HTTP status code.
/// The taxonomy always provides valid statuses, so this cannot happen for the
/// kinds shipped with the crate.
#[must_use]
pub fn to_payload(error: &AppError) -> ErrorResponse {
    let status = error.kind.http_status();
    let message = error.message.clone().unwrap_or_default().to_string();
    ErrorResponse::new(status, error.code.clone(), message)
        .expect("the taxonomy provides a valid status")
}

/// Renders the error as an RFC 7807 payload without the standard library.
#[must_use]
pub fn to_problem(error: AppError) -> ProblemJson {
    ProblemJson::from_app_error(error)
}

/// Returns the protocol mapping of a stable code.
#[must_use]
pub fn mapping_of(code: &AppCode) -> masterror::CodeMapping {
    mapping_for_code(code)
}

/// Describes how the error should be presented to the caller.
#[must_use]
pub fn presentation(error: &AppError) -> (DisplayMode, MessageEditPolicy, Option<RetryAdvice>) {
    let mode = match error.kind {
        AppErrorKind::Validation | AppErrorKind::BadRequest => DisplayMode::Prod,
        _ => DisplayMode::Staging
    };
    (mode, error.edit_policy, error.retry)
}

/// Parses the `#[error("...")]` template of a domain error.
#[must_use]
pub fn parse_template(source: &'static str) -> Option<usize> {
    let template = ErrorTemplate::parse(source).ok()?;
    Some(template.segments().len())
}

/// Returns the literal segments of a parsed template.
#[must_use]
pub fn template_literals(source: &'static str) -> Vec<&'static str> {
    let Ok(template) = ErrorTemplate::parse(source) else {
        return Vec::new();
    };
    template
        .segments()
        .iter()
        .filter_map(|segment| match segment {
            TemplateSegment::Literal(text) => Some(*text),
            TemplateSegment::Placeholder(_) => None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_the_domain_error() {
        let error = classify(SensorOutOfRange {
            sensor: "s-1",
            value:  23
        });
        assert_eq!(error.kind, AppErrorKind::Validation);
        assert_eq!(error.code, AppCode::Validation);
        assert!(error.to_string().contains("sensor s-1 reported 23"));
    }

    #[test]
    fn classifies_the_link_error() {
        let error = classify_link(LinkUnreachable {
            link: "bus-2"
        });
        assert_eq!(error.kind, AppErrorKind::Internal);
        assert_eq!(error.code, AppCode::Network);
        assert!(error.to_string().contains("link bus-2 is unreachable"));
    }

    #[test]
    fn builds_an_error_with_metadata() {
        let error = build_error();
        assert_eq!(error.kind, AppErrorKind::Conflict);
        assert_eq!(error.code, AppCode::Conflict);
        assert_eq!(
            error.retry,
            Some(RetryAdvice {
                after_seconds: 30
            })
        );
        assert!(error.metadata().get("phase").is_some());
        assert!(error.metadata().get("queue_depth").is_some());
    }

    #[test]
    fn attaches_a_source_error() {
        let error = attach_source(build_error());
        let source = error.source_ref().expect("the source is attached");
        assert!(source.to_string().contains("the sensor bus is offline"));
    }

    #[test]
    fn wraps_a_failed_result() {
        let error = wrap_result().expect_err("the result is a failure");
        assert_eq!(error.kind, AppErrorKind::Internal);
        assert!(
            error
                .render_message()
                .contains("the reading could not be taken")
        );
    }

    #[test]
    fn builds_a_context() {
        let error = wrap_with_context().expect_err("the result is a failure");
        assert_eq!(error.kind, AppErrorKind::Service);
        assert_eq!(error.code, AppCode::DependencyUnavailable);
        assert_eq!(error.edit_policy, MessageEditPolicy::Redact);
        assert!(error.metadata().get("phase").is_some());
    }

    #[test]
    fn renders_the_wire_payload() {
        let error = classify(SensorOutOfRange {
            sensor: "s-1",
            value:  23
        });
        let payload = to_payload(&error);
        assert_eq!(payload.status, AppErrorKind::Validation.http_status());
        assert_eq!(payload.code, AppCode::Validation);

        let problem = to_problem(classify(SensorOutOfRange {
            sensor: "s-1",
            value:  23
        }));
        assert_eq!(problem.status, payload.status);
        assert_eq!(problem.code, payload.code);
    }

    #[test]
    fn exposes_the_protocol_mapping() {
        let mapping = mapping_of(&AppCode::NotFound);
        assert_eq!(mapping.http_status(), 404);
        assert_eq!(mapping.kind(), AppErrorKind::NotFound);
        assert_eq!(mapping.grpc().name, "NOT_FOUND");
    }

    #[test]
    fn describes_the_presentation() {
        let error = build_error();
        let (mode, policy, retry) = presentation(&error);
        assert_eq!(mode, DisplayMode::Staging);
        assert_eq!(policy, MessageEditPolicy::Preserve);
        assert_eq!(
            retry,
            Some(RetryAdvice {
                after_seconds: 30
            })
        );
    }

    #[test]
    fn parses_the_error_template() {
        assert_eq!(parse_template("sensor {sensor} reported {value}"), Some(4));
        assert_eq!(
            template_literals("sensor {sensor} reported {value}"),
            vec!["sensor ", " reported "]
        );
    }
}
