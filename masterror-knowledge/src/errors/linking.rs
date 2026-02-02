// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Crate linking related errors.

mod e0460;
mod e0461;
mod e0462;
mod e0463;
mod e0464;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0460::ENTRY,
    &e0461::ENTRY,
    &e0462::ENTRY,
    &e0463::ENTRY,
    &e0464::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
