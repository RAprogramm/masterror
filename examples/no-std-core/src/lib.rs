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
//! Available in this configuration:
//!
//! - the derive macros for domain errors ([`derive@Error`]),
//! - the error taxonomy ([`AppErrorKind`]) and the stable codes ([`AppCode`]),
//! - the wire payloads ([`ErrorResponse`] and [`ProblemJson`]) with their
//!   numeric status field.
//!
//! Not available: the `http` interop. The `http` crate requires the standard
//! library, so the helpers returning `http::StatusCode` values stay behind the
//! `std` feature. The numeric status remains available through
//! [`AppErrorKind::http_status`].

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::string::ToString;

use masterror::{AppCode, AppError, AppErrorKind, Error, ErrorResponse, ProblemJson};

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

/// Converts a domain error into an [`AppError`] with the taxonomy attached.
#[must_use]
pub fn classify(reading: SensorOutOfRange) -> AppError {
    AppError::from(reading)
}

/// Renders the error as a wire payload without the standard library.
///
/// The status comes from the taxonomy, so the payload stays consistent with
/// the [`ProblemJson`] representation.
///
/// # Panics
///
/// Panics if the status of the error kind is not a valid HTTP status code.
/// The taxonomy always provides valid statuses, so this cannot happen for
/// the kinds shipped with the crate.
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
}
