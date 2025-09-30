#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    masterror_requires_error_generic_feature,
    feature(error_generic_member_access)
)]

//! Framework-agnostic application error types for backend services.
//!
//! # Overview
//!
//! A small, pragmatic error model designed for API-heavy services.  
//! The core is framework-agnostic; integrations are optional and enabled via
//! feature flags.
//!
//! Core types:
//! - [`AppError`] — rich error capturing code, taxonomy, message, metadata and
//!   transport hints
//! - [`AppErrorKind`] — stable internal taxonomy of application errors
//! - [`AppResult`] — convenience alias for returning [`AppError`]
//! - [`ProblemJson`] — RFC7807 payload emitted by HTTP/gRPC adapters
//! - [`ErrorResponse`] — legacy wire-level JSON payload for HTTP APIs
//! - [`AppCode`] — public, machine-readable error code for clients
//! - [`Metadata`] — structured telemetry attached to [`AppError`]
//! - [`field`] — helper functions to build [`Metadata`] without manual enums
//!
//! Key properties:
//! - Stable, predictable error categories (`AppErrorKind`).
//! - Explicit, overridable machine-readable codes (`AppCode`).
//! - Structured metadata for observability without ad-hoc `String` maps.
//! - Conservative and stable HTTP mappings.
//! - Internal error sources are never serialized to clients (only logged).
//! - Messages are safe to expose (human-oriented, non-sensitive).
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! MSRV is **1.90**. New minor releases may increase MSRV with a changelog
//! note, but never in a patch release.
//!
//! # Feature flags
//!
//! Enable only what you need:
//!
//! - `axum` — implements `IntoResponse` for [`AppError`] and [`ProblemJson`]
//!   with RFC7807 body
//! - `actix` — implements `Responder` for [`ProblemJson`] and Actix
//!   `ResponseError` for [`AppError`]
//! - `tonic` — converts [`struct@Error`] into `tonic::Status` with sanitized
//!   metadata
//! - `openapi` — derives an OpenAPI schema for [`ErrorResponse`] (via `utoipa`)
//! - `sqlx` — `From<sqlx::Error>` mapping
//! - `redis` — `From<redis::RedisError>` mapping
//! - `validator` — `From<validator::ValidationErrors>` mapping
//! - `config` — `From<config::ConfigError>` mapping
//! - `tokio` — `From<tokio::time::error::Elapsed>` mapping
//! - `reqwest` — `From<reqwest::Error>` mapping
//! - `teloxide` — `From<teloxide_core::RequestError>` mapping
//! - `telegram-webapp-sdk` —
//!   `From<telegram_webapp_sdk::utils::validate_init_data::ValidationError>`
//!   mapping
//! - `frontend` — convert errors into `wasm_bindgen::JsValue` and emit
//!   `console.error` logs in WASM/browser contexts
//! - `serde_json` — support for structured JSON details in [`ErrorResponse`]
//!   and [`ProblemJson`]; also pulled transitively by `axum`
//! - `multipart` — compatibility flag for Axum multipart
//! - `turnkey` — domain taxonomy and conversions for Turnkey errors, exposed in
//!   the `turnkey` module
//!
//! # Derive macros and telemetry
//!
//! The [`masterror::Error`](derive@crate::Error) derive mirrors `thiserror`
//! while adding `#[app_error]` and `#[provide]` attributes. Annotate your
//! domain errors once to surface structured telemetry via
//! [`std::error::Request`] and generate conversions into [`AppError`] /
//! [`AppCode`].
//!
//! ```rust
//! use masterror::{AppCode, AppError, AppErrorKind, Error};
//!
//! #[derive(Debug, Error)]
//! #[error("missing flag: {name}")]
//! #[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
//! struct MissingFlag {
//!     name: &'static str
//! }
//!
//! let app: AppError = MissingFlag {
//!     name: "feature"
//! }
//! .into();
//! assert!(matches!(app.kind, AppErrorKind::BadRequest));
//! ```
//!
//! Use `#[provide]` to forward typed telemetry that downstream consumers can
//! extract from [`AppError`] via `std::error::Request`.
//!
//! ## Masterror derive: end-to-end domain errors
//!
//! `#[derive(Masterror)]` builds on top of `#[derive(Error)]`, wiring a domain
//! error directly into [`struct@crate::Error`] with typed telemetry, redaction
//! policy and transport hints. The `#[masterror(...)]` attribute mirrors the
//! `thiserror` style while keeping redaction decisions and metadata in one
//! place.
//!
//! ```rust
//! use masterror::{
//!     AppCode, AppErrorKind, Error, Masterror, MessageEditPolicy, mapping::HttpMapping
//! };
//!
//! #[derive(Debug, Masterror)]
//! #[error("user {user_id} missing flag {flag}")]
//! #[masterror(
//!     code = AppCode::NotFound,
//!     category = AppErrorKind::NotFound,
//!     message,
//!     redact(message, fields("user_id" = hash)),
//!     telemetry(
//!         Some(masterror::field::str("user_id", user_id.clone())),
//!         attempt.map(|value| masterror::field::u64("attempt", value))
//!     ),
//!     map.grpc = 5,
//!     map.problem = "https://errors.example.com/not-found"
//! )]
//! struct MissingFlag {
//!     user_id: String,
//!     flag:    &'static str,
//!     attempt: Option<u64>,
//!     #[source]
//!     source:  Option<std::io::Error>
//! }
//!
//! let err = MissingFlag {
//!     user_id: "alice".into(),
//!     flag:    "beta",
//!     attempt: Some(2),
//!     source:  None
//! };
//! let converted: Error = err.into();
//! assert_eq!(converted.code, AppCode::NotFound);
//! assert_eq!(converted.kind, AppErrorKind::NotFound);
//! assert_eq!(converted.edit_policy, MessageEditPolicy::Redact);
//! assert!(converted.metadata().get("user_id").is_some());
//! assert_eq!(
//!     MissingFlag::HTTP_MAPPING,
//!     HttpMapping::new(AppCode::NotFound, AppErrorKind::NotFound)
//! );
//! ```
//!
//! - `code` — public [`AppCode`].
//! - `category` — semantic [`AppErrorKind`].
//! - `message` — expose the formatted [`core::fmt::Display`] output as the
//!   public message.
//! - `redact(message)` — mark the message as redactable at the transport
//!   boundary, `fields("name" = hash, "card" = last4)` override metadata
//!   policies (`hash`, `last4`, `redact`, `none`).
//! - `telemetry(...)` — list of expressions producing
//!   `Option<masterror::Field>` to be inserted into [`Metadata`].
//! - `map.grpc` / `map.problem` — optional gRPC status (as `i32`) and
//!   problem+json type for generated mapping tables. Access them via
//!   `TYPE::HTTP_MAPPING`, `TYPE::GRPC_MAPPING`/`MAPPINGS` and
//!   `TYPE::PROBLEM_MAPPING`/`MAPPINGS`.
//!
//! The derive continues to honour `#[from]`, `#[source]` and `#[backtrace]`
//! field attributes, automatically attaching sources and captured backtraces to
//! the resulting [`struct@Error`].
//!
//! # Domain integrations: Turnkey
//!
//! With the `turnkey` feature enabled, the crate exports a `turnkey` module
//! that provides:
//!
//! - `turnkey::TurnkeyErrorKind` — stable categories for Turnkey-specific
//!   failures
//! - `turnkey::TurnkeyError` — a container with `kind` and safe, public message
//! - `turnkey::classify_turnkey_error` — heuristic classifier for raw
//!   SDK/provider strings
//! - conversions: `From<TurnkeyError>` → [`AppError`] and
//!   `From<TurnkeyErrorKind>` → [`AppErrorKind`]
//!
//! ## Example
//!
//! ```rust
//! # #[cfg(feature = "turnkey")]
//! # {
//! use masterror::{
//!     AppError, AppErrorKind,
//!     turnkey::{TurnkeyError, TurnkeyErrorKind, classify_turnkey_error}
//! };
//!
//! // Classify a raw provider message
//! let kind = classify_turnkey_error("429 Too Many Requests");
//! assert!(matches!(kind, TurnkeyErrorKind::RateLimited));
//!
//! // Build and convert into AppError
//! let e = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "throttled by upstream");
//! let app: AppError = e.into();
//! assert_eq!(app.kind, AppErrorKind::RateLimited);
//! # }
//! ```
//!
//! # Error taxonomy
//!
//! Applications convert domain/infrastructure failures into [`AppError`] with a
//! semantic [`AppErrorKind`] and optional public message:
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind};
//!
//! let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
//! assert!(matches!(err.kind, AppErrorKind::BadRequest));
//! ```
//!
//! Attach structured metadata for telemetry and logging:
//! ```rust
//! use masterror::{AppError, AppErrorKind, field};
//!
//! let err = AppError::service("downstream degraded")
//!     .with_field(field::str("request_id", "abc123"))
//!     .with_field(field::i64("attempt", 2));
//! assert_eq!(err.metadata().len(), 2);
//! ```
//!
//! Attach upstream diagnostics without cloning existing `Arc`s:
//! ```rust
//! use masterror::AppError;
//!
//! let err = AppError::internal("db down")
//!     .with_context(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
//! assert!(err.source_ref().is_some());
//! ```
//!
//! [`AppErrorKind`] controls the default HTTP status mapping.  
//! [`AppCode`] provides a stable machine-readable code for clients.  
//! Together, they form the wire contract in [`ErrorResponse`].
//!
//! # Wire payload: [`ErrorResponse`]
//!
//! The stable JSON payload for HTTP APIs contains:
//! - `status: u16` — HTTP status code
//! - `code: AppCode` — stable machine-readable error code
//! - `message: String` — human-friendly, safe-to-expose text
//! - `details` — optional details (JSON if `serde_json`, otherwise string)
//! - `retry` — optional retry advice (`Retry-After`)
//! - `www_authenticate` — optional authentication challenge
//!
//! Example construction:
//!
//! ```rust
//! use masterror::{AppCode, ErrorResponse};
//!
//! let resp = ErrorResponse::new(404, AppCode::NotFound, "User not found").expect("status");
//! ```
//!
//! Conversion from [`AppError`]:
//!
//! ```rust
//! use masterror::{AppCode, AppError, AppErrorKind, ErrorResponse};
//!
//! let app_err = AppError::new(AppErrorKind::NotFound, "user_not_found");
//! let resp: ErrorResponse = (&app_err).into();
//! assert_eq!(resp.status, 404);
//! assert_eq!(resp.code, AppCode::NotFound);
//! ```
//!
//! # Typed control-flow macros
//!
//! Reach for [`ensure!`] and [`fail!`] when you need to exit early with a typed
//! error without paying for string formatting or heap allocations on the
//! success path.
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind, AppResult};
//!
//! fn guard(flag: bool) -> AppResult<()> {
//!     masterror::ensure!(flag, AppError::bad_request("flag must be set"));
//!     Ok(())
//! }
//!
//! fn bail() -> AppResult<()> {
//!     masterror::fail!(AppError::unauthorized("token expired"));
//! }
//!
//! assert!(guard(true).is_ok());
//! assert!(matches!(
//!     guard(false).unwrap_err().kind,
//!     AppErrorKind::BadRequest
//! ));
//! assert!(matches!(
//!     bail().unwrap_err().kind,
//!     AppErrorKind::Unauthorized
//! ));
//! ```
//!
//! # Axum integration
//!
//! With the `axum` feature enabled, you can return [`AppError`] directly from
//! handlers. It is automatically converted into an [`ErrorResponse`] JSON
//! payload.
//!
//! ```rust,ignore
//! use axum::{routing::get, Router};
//! use masterror::{AppError, AppResult};
//!
//! async fn handler() -> AppResult<&'static str> {
//!     Err(AppError::forbidden("No access"))
//! }
//!
//! let app = Router::new().route("/demo", get(handler));
//! ```
//!
//! # OpenAPI integration
//!
//! With the `openapi` feature enabled, [`ErrorResponse`] derives
//! `utoipa::ToSchema` and can be referenced in OpenAPI operation responses.
//!
//! # Versioning policy
//!
//! This crate follows semantic versioning. Any change to the public API
//! or wire contract is considered a **breaking change** and requires a major
//! version bump.
//!
//! # Safety
//!
//! This crate does not use `unsafe`.
//!
//! # License
//!
//! Licensed under either of
//! - Apache License, Version 2.0
//! - MIT license
//!
//! at your option.

extern crate alloc;

mod app_error;
mod code;
mod convert;
pub mod error;
mod kind;
mod macros;
#[cfg(masterror_has_error_generic_member_access)]
#[doc(hidden)]
pub mod provide;
mod response;
mod result_ext;

#[cfg(feature = "frontend")]
#[cfg_attr(docsrs, doc(cfg(feature = "frontend")))]
pub mod frontend;

#[cfg(feature = "turnkey")]
#[cfg_attr(docsrs, doc(cfg(feature = "turnkey")))]
pub mod turnkey;

/// Minimal prelude re-exporting core types for handler signatures.
pub mod prelude;

/// Transport mapping descriptors for generated domain errors.
pub mod mapping;

pub use app_error::{
    AppError, AppResult, Context, Error, Field, FieldRedaction, FieldValue, MessageEditPolicy,
    Metadata, field
};
pub use code::{AppCode, ParseAppCodeError};
pub use kind::AppErrorKind;
/// Re-export derive macros so users only depend on this crate.
///
/// # Examples
///
/// ```
/// use masterror::{AppCode, AppError, AppErrorKind, Error};
///
/// #[derive(Debug, Error)]
/// #[error("missing flag: {name}")]
/// #[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
/// struct MissingFlag {
///     name: &'static str
/// }
///
/// let app: AppError = MissingFlag {
///     name: "feature"
/// }
/// .into();
/// assert!(matches!(app.kind, AppErrorKind::BadRequest));
///
/// let code: AppCode = MissingFlag {
///     name: "other"
/// }
/// .into();
/// assert_eq!(code, AppCode::BadRequest);
/// ```
pub use masterror_derive::{Error, Masterror};
pub use response::{
    ErrorResponse, ProblemJson, RetryAdvice,
    problem_json::{
        CODE_MAPPINGS, CodeMapping, GrpcCode, ProblemMetadata, ProblemMetadataValue,
        mapping_for_code
    }
};
pub use result_ext::ResultExt;

#[cfg(feature = "tonic")]
#[cfg_attr(docsrs, doc(cfg(feature = "tonic")))]
pub use crate::convert::StatusConversionError;
