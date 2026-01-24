// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Borrowing-related errors.

mod e0499;
mod e0500;
mod e0501;
mod e0502;
mod e0503;
mod e0506;
mod e0508;
mod e0596;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0499::ENTRY,
    &e0500::ENTRY,
    &e0501::ENTRY,
    &e0502::ENTRY,
    &e0503::ENTRY,
    &e0506::ENTRY,
    &e0508::ENTRY,
    &e0596::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
