// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Name resolution errors.

mod e0412;
mod e0425;
mod e0433;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0412::ENTRY, &e0425::ENTRY, &e0433::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
