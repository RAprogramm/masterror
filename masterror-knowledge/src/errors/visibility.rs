// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Visibility related errors.

mod e0364;
mod e0365;
mod e0445;
mod e0446;
mod e0447;
mod e0448;
mod e0449;
mod e0451;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0364::ENTRY,
    &e0365::ENTRY,
    &e0445::ENTRY,
    &e0446::ENTRY,
    &e0447::ENTRY,
    &e0448::ENTRY,
    &e0449::ENTRY,
    &e0451::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
