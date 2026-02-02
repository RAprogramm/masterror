// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Pattern matching errors.

mod e0004;
mod e0005;
mod e0023;
mod e0025;
mod e0026;
mod e0027;
mod e0029;
mod e0030;
mod e0033;
mod e0158;
mod e0164;
mod e0170;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0004::ENTRY,
    &e0005::ENTRY,
    &e0023::ENTRY,
    &e0025::ENTRY,
    &e0026::ENTRY,
    &e0027::ENTRY,
    &e0029::ENTRY,
    &e0030::ENTRY,
    &e0033::ENTRY,
    &e0158::ENTRY,
    &e0164::ENTRY,
    &e0170::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
