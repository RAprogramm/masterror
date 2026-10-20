// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

//! Shared helpers for error derive macros.
//!
//! This crate exposes the formatting template parser used by `masterror`
//! to interpret `#[error("...")]` attributes. It is internal to the
//! workspace but kept separate so that procedural macros can reuse the
//! parsing logic without a circular dependency.
//!
//! The crate is `no_std` compatible: it only relies on `core` and `alloc`,
//! so it can be used from crates that target bare-metal platforms.

extern crate alloc;

pub mod template;
