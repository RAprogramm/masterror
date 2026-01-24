// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Type-related errors.

mod e0277;
mod e0308;
mod e0599;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0277::ENTRY, &e0308::ENTRY, &e0599::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
