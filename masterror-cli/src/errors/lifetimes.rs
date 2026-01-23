// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Lifetime-related errors.

mod e0106;
mod e0495;
mod e0515;
mod e0597;
mod e0621;
mod e0623;
mod e0700;
mod e0716;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0106::ENTRY,
    &e0495::ENTRY,
    &e0515::ENTRY,
    &e0597::ENTRY,
    &e0621::ENTRY,
    &e0623::ENTRY,
    &e0700::ENTRY,
    &e0716::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
