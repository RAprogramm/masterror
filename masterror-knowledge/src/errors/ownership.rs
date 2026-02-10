// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Ownership-related errors.

mod e0373;
mod e0381;
mod e0382;
mod e0383;
mod e0384;
mod e0505;
mod e0507;
mod e0509;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0373::ENTRY,
    &e0381::ENTRY,
    &e0382::ENTRY,
    &e0383::ENTRY,
    &e0384::ENTRY,
    &e0505::ENTRY,
    &e0507::ENTRY,
    &e0509::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
