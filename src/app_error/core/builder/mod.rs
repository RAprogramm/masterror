// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Builder pattern implementation for `AppError`.
//!
//! This module provides fluent builder methods for constructing errors with
//! various attributes like metadata, source chains, and diagnostics.

mod constructors;
mod context;
mod details;
mod diagnostics;
mod metadata;
mod modifiers;

#[cfg(test)]
mod tests;
