//! Utilities for building custom error derive infrastructure.
//!
//! This module exposes lower-level building blocks that will eventually power
//! a native replacement for the `thiserror` derive. The initial goal is to
//! parse and validate display templates (`#[error("...")]`) in a reusable
//! and well-tested manner so that future procedural macros can focus on
//! generating code.
//!
//! The API is intentionally low level. It makes no assumptions about how the
//! parsed data is going to be used and instead provides precise spans and
//! formatting metadata that higher-level code can rely on.

/// Parser and formatter helpers for `#[error("...")]` templates.
pub mod template;
