//! Framework-agnostic application error types for backend services.
//!
//! # Overview
//!
//! A small, pragmatic error model designed for API-heavy services.  
//! The core is framework-agnostic; integrations are optional and enabled via
//! feature flags.
//!
//! Core types:
//! - [`AppError`] — thin wrapper around a semantic error kind and optional
//!   message
//! - [`AppErrorKind`] — stable internal taxonomy of application errors
//! - [`AppResult`] — convenience result alias (defaults to [`AppError`])
//! - [`ErrorResponse`] — stable wire-level JSON payload for HTTP APIs
//! - [`AppCode`] — public, machine-readable error code for clients
//!
//! Key properties:
//! - Stable, predictable error categories (`AppErrorKind`).
//! - Conservative and stable HTTP mappings.
//! - Internal error sources are never serialized to clients (only logged).
//! - Messages are safe to expose (human-oriented, non-sensitive).
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! MSRV is **1.89**. New minor releases may increase MSRV with a changelog
//! note, but never in a patch release.
//!
//! # Feature flags
//!
//! Enable only what you need:
//!
//! - `axum` — implements `IntoResponse` for [`AppError`] and [`ErrorResponse`]
//!   with JSON body
//! - `actix` — implements `Responder` for [`ErrorResponse`] (and Actix
//!   integration for [`AppError`])
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
//! - `serde_json` — support for structured JSON details in [`ErrorResponse`];
//!   also pulled transitively by `axum`
//! - `multipart` — compatibility flag for Axum multipart
//! - `turnkey` — domain taxonomy and conversions for Turnkey errors, exposed in
//!   the `turnkey` module
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
//! assert!(matches!(resp.code, AppCode::NotFound));
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

#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all
)]
// Show feature-gated items on docs.rs
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod app_error;
mod code;
mod convert;
mod kind;
mod response;

#[cfg(feature = "frontend")]
#[cfg_attr(docsrs, doc(cfg(feature = "frontend")))]
pub mod frontend;

#[cfg(feature = "turnkey")]
#[cfg_attr(docsrs, doc(cfg(feature = "turnkey")))]
pub mod turnkey;

/// Minimal prelude re-exporting core types for handler signatures.
pub mod prelude;

pub use app_error::{AppError, AppResult};
pub use code::AppCode;
pub use kind::AppErrorKind;
pub use response::{ErrorResponse, RetryAdvice};
