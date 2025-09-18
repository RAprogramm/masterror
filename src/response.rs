//! Wire-level error payload and HTTP integration.
//!
//! # Purpose
//!
//! [`ErrorResponse`] is a stable JSON structure intended to be returned
//! directly from HTTP handlers. It represents the **public-facing contract**
//! for error reporting in web APIs.
//!
//! It deliberately contains only *safe-to-expose* fields:
//!
//! - [`status`](ErrorResponse::status): HTTP status code chosen by the service
//! - [`code`](ErrorResponse::code): stable, machine-readable error code
//!   ([`AppCode`])
//! - [`message`](ErrorResponse::message): human-oriented, non-sensitive text
//! - [`details`](ErrorResponse::details): optional structured payload
//!   (`serde_json::Value` if the `serde_json` feature is enabled, otherwise
//!   plain text)
//! - [`retry`](ErrorResponse::retry): optional retry advice, rendered as the
//!   `Retry-After` header in HTTP adapters; set via
//!   [`with_retry_after_secs`](ErrorResponse::with_retry_after_secs) or
//!   [`with_retry_after_duration`](ErrorResponse::with_retry_after_duration)
//! - [`www_authenticate`](ErrorResponse::www_authenticate): optional
//!   authentication challenge string, rendered as the `WWW-Authenticate` header
//!
//! Internal error sources (the [`std::error::Error`] chain inside [`AppError`])
//! are **never leaked** into this type. They should be logged at the boundary,
//! but not serialized into responses.
//!
//! # Example
//!
//! ```rust
//! use std::time::Duration;
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
mod legacy;
mod mapping;
mod metadata;

#[cfg(feature = "axum")]
mod axum_impl;

#[cfg(feature = "actix")]
mod actix_impl;

pub use core::{ErrorResponse, RetryAdvice};

#[cfg(test)]
mod tests;
