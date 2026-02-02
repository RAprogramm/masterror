// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Const evaluation related errors.

mod e0492;
mod e0493;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0492::ENTRY, &e0493::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
