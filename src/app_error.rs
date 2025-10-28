// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Core application error type: [`AppError`].
//!
//! [`AppError`] is a thin, framework-agnostic wrapper around a canonical
//! error taxonomy [`AppErrorKind`] plus an optional public-facing message.
//! The `Display` for `AppError` prints only the kind, not the message, to keep
//! logs and errors concise by default.
//!
//! ## Design
//!
//! - **Stable taxonomy:** the semantic category is captured by
//!   [`AppErrorKind`].
//! - **Optional message:** human-readable, safe-to-expose text. Do not put
//!   secrets here.
//! - **Structured metadata:** attach typed key/value pairs for diagnostics via
//!   [`Metadata`].
//! - **No panics:** all helpers avoid `unwrap/expect`.
//! - **Transport-agnostic:** mapping to HTTP lives in `kind.rs` and
//!   `convert/*`.
//!
//! ## Common usage
//!
//! Build errors either with generic constructors or named helpers matching
//! taxonomy variants:
//!
//! ```rust
//! use masterror::{AppError, AppErrorKind};
//!
//! // generic
//! let e1 = AppError::with(AppErrorKind::BadRequest, "flag_required");
//! let e2 = AppError::new(AppErrorKind::Forbidden, "access denied");
//!
//! // named helpers
//! let e3 = AppError::not_found("user not found");
//! let e4 = AppError::timeout("operation timed out");
//!
//! assert!(matches!(e1.kind, AppErrorKind::BadRequest));
//! assert!(e3.message.as_deref() == Some("user not found"));
//! ```
//!
//! ## HTTP (Axum) integration
//!
//! With the `axum` feature enabled the crate provides `IntoResponse` for
//! `AppError` (see `convert/axum.rs`). You can return `AppResult<T>` from
//! handlers and the crate will build a JSON error (if `serde_json` is enabled)
//! with status derived from [`AppErrorKind`].
//!
//! ```rust,ignore
//! # #[cfg(feature = "axum")]
//! use masterror::{AppError, AppResult};
//!
//! async fn handler() -> AppResult<&'static str> {
//!     Err(AppError::forbidden("no access"))
//! }
//! ```
//!
//! ## Telemetry
//!
//! [`AppError::log`] flushes telemetry once: it emits a structured `tracing`
//! event (when the `tracing` feature is enabled), increments the
//! `error_total{code,category}` counter (with the `metrics` feature) and
//! captures a lazy [`Backtrace`] snapshot (with the `backtrace` feature).
//! Constructors and framework integrations call it automatically, so manual
//! usage is rarely required.

mod constructors;
mod context;
mod core;
mod metadata;

pub use core::{AppError, AppResult, DisplayMode, Error, ErrorChain, MessageEditPolicy};
#[cfg(all(test, feature = "backtrace"))]
pub(crate) use core::{reset_backtrace_preference, set_backtrace_preference_override};

pub use context::Context;
pub(crate) use metadata::duration_to_string;
pub use metadata::{Field, FieldRedaction, FieldValue, Metadata, field};

#[cfg(test)]
mod tests;
