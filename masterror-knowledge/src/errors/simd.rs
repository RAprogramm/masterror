// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! SIMD related errors.

mod e0075;
mod e0076;
mod e0077;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0075::ENTRY, &e0076::ENTRY, &e0077::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
