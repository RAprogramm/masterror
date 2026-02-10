// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Unsafe code related errors.

mod e0133;
mod e0197;
mod e0198;
mod e0199;
mod e0200;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0133::ENTRY,
    &e0197::ENTRY,
    &e0198::ENTRY,
    &e0199::ENTRY,
    &e0200::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
