//! Minimal, opt-in prelude for application crates.
//!
//! Import this prelude in HTTP handlers and services to keep signatures tidy:
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
//! The prelude intentionally re-exports only the core types used in everyday
//! application code. Optional integrations (e.g. `IntoResponse`) are enabled
//! via feature flags on the crate and do not require additional imports.

/// Re-export the core error type.
pub use crate::AppError;
/// Re-export the high-level error taxonomy.
pub use crate::AppErrorKind;
/// Re-export the conventional result alias used in handlers/services.
pub use crate::AppResult;
/// Re-export the stable wire-level error payload type for HTTP APIs.
pub use crate::ErrorResponse;
