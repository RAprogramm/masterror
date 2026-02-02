// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Feature flag and compiler feature errors.

mod e0635;
mod e0636;
mod e0658;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[&e0635::ENTRY, &e0636::ENTRY, &e0658::ENTRY];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
