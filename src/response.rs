// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Wire-level error payload and HTTP integration.
//!
//! # Purpose
//!
//! [`ProblemJson`] serializes an RFC7807 payload designed for HTTP responses.
//! It augments the legacy [`ErrorResponse`] (still available for manual usage)
//! with:
//!
//! - canonical problem `type` URIs derived from [`AppCode`]
//! - a `title` computed from [`AppErrorKind`]
//! - the stable machine code plus optional gRPC mapping (`grpc.code`,
//!   `grpc.value`)
//! - retry/authentication hints surfaced via the `Retry-After` and
//!   `WWW-Authenticate` headers
//! - sanitized [`Metadata`] values when the error is not marked redactable
//!
//! When the message is tagged redactable (`AppError::redactable` or
//! `Context::redact(true)`), both `detail` and metadata are omitted to avoid
//! leaking sensitive information. The HTTP adapters (`axum`, `actix`) emit
//! `application/problem+json` bodies automatically via [`ProblemJson`].
//!
//! [`ErrorResponse`] remains available for backwards compatibility with
//! existing wire contracts and can be converted into [`ProblemJson`] via
//! [`ProblemJson::from_error_response`].
//!
//! # Example
//!
//! ```rust
//! use core::time::Duration;
//!
//! use masterror::{AppCode, ErrorResponse};
//!
//! let resp = ErrorResponse::new(404, AppCode::NotFound, "User not found")
//!     .expect("status")
//!     .with_retry_after_duration(Duration::from_secs(30));
//! ```
//!
//! With `serde_json` enabled:
//!
//! ```rust
//! # #[cfg(feature = "serde_json")]
//! # {
//! use masterror::{AppCode, ErrorResponse};
//! use serde_json::json;
//!
//! let resp = ErrorResponse::new(422, AppCode::Validation, "Invalid input")
//!     .expect("status")
//!     .with_details_json(json!({"field": "email", "error": "invalid"}));
//! # }
//! ```
//!
//! # Migration note
//!
//! Prior to version 0.3.0, `ErrorResponse::new` accepted only `(status,
//! message)`. This was replaced with `(status, code, message)` to expose a
//! stable machine-readable code. A temporary [`ErrorResponse::new_legacy`] is
//! provided as a deprecated shim.

mod core;
mod details;
pub mod internal;
mod legacy;
mod mapping;
mod metadata;
pub mod problem_json;

#[cfg(feature = "axum")]
mod axum_impl;

#[cfg(feature = "actix")]
pub(crate) mod actix_impl;

pub use core::{ErrorResponse, RetryAdvice};

pub use problem_json::ProblemJson;

#[cfg(test)]
mod tests;
