// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Display formatting for `AppError`.
//!
//! Provides environment-aware display modes: production (compact JSON),
//! local (human-readable), and staging (JSON with context).

mod helpers;
mod local;
mod mode;
mod prod;
mod staging;

#[cfg(test)]
mod tests;

pub use mode::DisplayMode;
