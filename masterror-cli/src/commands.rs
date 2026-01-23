// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! CLI commands.

mod check;
mod explain;
mod list;
pub mod practice;

pub use check::run as check;
pub use explain::run as explain;
pub use list::run as list;
