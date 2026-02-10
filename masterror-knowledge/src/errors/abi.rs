// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! ABI and FFI related errors.

mod e0044;
mod e0045;
mod e0060;
mod e0130;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0044::ENTRY, &e0045::ENTRY, &e0060::ENTRY, &e0130::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
