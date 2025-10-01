// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Turnkey integration: error kinds, classification, and conversions.
//!
//! This module centralizes Turnkey-specific error taxonomy and mapping into
//! framework-agnostic [`crate::AppError`] and [`crate::AppErrorKind`].
//!
//! # Goals
//! - Stable domain kinds (`TurnkeyErrorKind`) decoupled from SDK texts.
//! - Conservative mapping to the canonical [`crate::AppErrorKind`].
//! - Heuristic classifier for stringly-typed upstream errors.
//!
//! # Examples
//!
//! ```rust
//! use masterror::{
//!     AppError, AppErrorKind,
//!     turnkey::{TurnkeyError, TurnkeyErrorKind, classify_turnkey_error}
//! };
//!
//! // Construct a domain error
//! let e = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "429 from upstream");
//!
//! // Convert into AppError for transport mapping
//! let app: AppError = e.clone().into();
//! assert_eq!(app.kind, AppErrorKind::RateLimited);
//!
//! // Classify raw SDK message
//! let k = classify_turnkey_error("label must be unique");
//! assert!(matches!(k, TurnkeyErrorKind::UniqueLabel));
//! ```

mod classifier;
mod conversions;
mod domain;

pub use classifier::classify_turnkey_error;
pub use domain::{TurnkeyError, TurnkeyErrorKind, map_turnkey_kind};

#[cfg(test)]
mod tests;
