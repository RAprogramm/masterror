// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Generic parameter related errors.

mod e0107;
mod e0109;
mod e0128;
mod e0393;
mod e0401;
mod e0403;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0107::ENTRY,
    &e0109::ENTRY,
    &e0128::ENTRY,
    &e0393::ENTRY,
    &e0401::ENTRY,
    &e0403::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
