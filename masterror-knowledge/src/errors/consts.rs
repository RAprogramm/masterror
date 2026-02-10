// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Constant and static related errors.

mod e0010;
mod e0015;
mod e0080;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0010::ENTRY, &e0015::ENTRY, &e0080::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
