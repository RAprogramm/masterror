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

/// Re-export the stable machine-readable error code.
pub use crate::AppCode;
/// Re-export the core application error type.
pub use crate::AppError;
/// Re-export the high-level error taxonomy (stable categories).
pub use crate::AppErrorKind;
/// Re-export the conventional result alias used in handlers/services.
pub use crate::AppResult;
/// Re-export the stable wire-level error payload for HTTP APIs.
pub use crate::ErrorResponse;
