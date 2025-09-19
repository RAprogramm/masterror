//! Shared helpers for error derive macros.
//!
//! This crate exposes the formatting template parser used by `masterror`
//! to interpret `#[error("...")]` attributes. It is internal to the
//! workspace but kept separate so that procedural macros can reuse the
//! parsing logic without a circular dependency.

pub mod template;
