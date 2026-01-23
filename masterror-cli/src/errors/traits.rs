// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Trait-related errors.

mod e0038;
mod e0282;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0038::ENTRY, &e0282::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
