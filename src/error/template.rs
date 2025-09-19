//! Parser and formatter helpers for `#[error("...")]` attributes.
//!
//! This module re-exports the shared helpers from the internal
//! `masterror_template` crate so that downstream code can continue using the
//! stable path `masterror::error::template`.

pub use masterror_template::template::*;
