// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Parser and formatter helpers for `#[error("...")]` attributes.
//!
//! This module re-exports the shared helpers from the internal
//! `masterror_template` crate so that downstream code can continue using the
//! stable path `masterror::error::template`.

pub use masterror_template::template::*;
