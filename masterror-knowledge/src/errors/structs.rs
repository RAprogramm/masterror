// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Struct-related errors.

mod e0124;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0124::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
