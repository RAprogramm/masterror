//! Framework-agnostic application error types for backend services.
//!
//! # Overview
//!
//! This crate provides a small, pragmatic error model for API-heavy services:
//!
//! - Core types: [`AppError`], [`AppErrorKind`], [`AppResult`],
//!   [`ErrorResponse`].
//! - Strictly framework-agnostic core with optional integrations behind feature
//!   flags.
//! - Optional Axum integration (`IntoResponse`) behind the `axum` feature.
//! - Optional OpenAPI schema for the wire error payload behind the `openapi`
//!   feature.
//!
//! The design favors stable, machine-readable categories ([`AppErrorKind`]) and
//! conservative HTTP mappings. Internals are never leaked into wire payloads;
//! messages are public-friendly; sources are logged only.
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! MSRV is **1.89**. New minor releases may raise MSRV with a changelog note,
//! but not in a patch release.
//!
//! # Feature flags
//!
//! Enable only what you need:
//!
//! - `axum` — implements `IntoResponse` for [`AppError`] and uses JSON body.
//! - `actix` — implements `actix_web::ResponseError` for [`AppError`] and
//!   JSON-response.
//! - `openapi` — derives OpenAPI schema for [`ErrorResponse`] (via `utoipa`).
//! - `sqlx` — `From<sqlx::Error>` mapping.
//! - `redis` — `From<redis::RedisError>` mapping.
//! - `validator` — `From<validator::ValidationErrors>` mapping.
//! - `config` — `From<config::ConfigError>` mapping.
//! - `tokio` — `From<tokio::time::error::Elapsed>` mapping.
//! - `reqwest` — `From<reqwest::Error>` mapping.
//! - `serde_json` — opt-in for JSON details; also pulled transitively by
//!   `axum`.
//! - `multipart` — kept for projects compiling Axum with multipart.
//!
//! # Error taxonomy
//!
//! Applications convert domain and infrastructure failures into [`AppError`]
//! with an [`AppErrorKind`] and an optional human-readable message:
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind};
//!
//! let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
//! assert!(matches!(err.kind, AppErrorKind::BadRequest));
//! ```
//!
//! [`AppErrorKind`] controls the default HTTP status (when the `axum` feature
//! is enabled).
//!
//! # Axum integration
//!
//! Enable `axum` to return errors directly from handlers:
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
//! The wire payload is [`ErrorResponse`]. The internal source chain (if any)
//! is logged, but not serialized. This avoids leaking internals to clients.
//!
//! # OpenAPI
//!
//! With `openapi`, [`ErrorResponse`] derives a schema and can be referenced
//! in your operation responses.
//!
//! # Versioning policy
//!
//! The crate follows semantic versioning. Changing wire contract semantics
//! or public API is considered a breaking change.
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
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all
)]
// Show feature-gated items on docs.rs
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod app_error;
mod convert;
mod kind;
mod response;

/// Minimal prelude re-exporting core types used in handlers/services.
pub mod prelude;

pub use app_error::{AppError, AppResult};
pub use kind::AppErrorKind;
pub use response::ErrorResponse;
