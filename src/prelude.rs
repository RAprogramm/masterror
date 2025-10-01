// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Minimal, opt-in prelude for application crates.
//!
//! # Purpose
//!
//! This module provides a small set of re-exports for the most common types
//! when working with application errors.  
//! Importing this prelude keeps handler and service signatures concise without
//! polluting the namespace with rarely used items.
//!
//! # Example
//!
//! ```rust
//! use masterror::prelude::*;
//!
//! fn demo(flag: bool) -> AppResult<()> {
//!     if !flag {
//!         return Err(AppError::new(AppErrorKind::BadRequest, "flag_required"));
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Design notes
//!
//! - Only the **core error types** are re-exported here.
//! - Optional framework integrations (e.g. `IntoResponse` for Axum, Actix
//!   `Responder`) remain gated behind feature flags and do not require explicit
//!   imports from this prelude.
//! - This keeps the public surface small, predictable, and easy to reason
//!   about.

/// Stable machine-readable error code used in wire contracts.
pub use crate::AppCode;
/// Core application error type (`kind` + optional message).
pub use crate::AppError;
/// High-level taxonomy of application errors (stable categories).
pub use crate::AppErrorKind;
/// Convenience alias for returning [`AppError`] from handlers/services.
pub use crate::AppResult;
/// Stable wire-level error payload for HTTP APIs.
pub use crate::ErrorResponse;
/// Turnkey-specific error taxonomy and helpers (enabled with the `turnkey`
/// feature).
///
/// Re-exports:
/// - [`TurnkeyErrorKind`] — stable categories for Turnkey-specific failures
/// - [`TurnkeyError`] — error container with kind + public message
/// - [`classify_turnkey_error`] — heuristic classifier for raw SDK/provider
///   strings
#[cfg(feature = "turnkey")]
pub use crate::turnkey::{TurnkeyError, TurnkeyErrorKind, classify_turnkey_error};
