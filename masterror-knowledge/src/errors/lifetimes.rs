// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Lifetime-related errors.

mod e0106;
mod e0195;
mod e0226;
mod e0227;
mod e0228;
mod e0261;
mod e0262;
mod e0263;
mod e0478;
mod e0482;
mod e0491;
mod e0495;
mod e0496;
mod e0515;
mod e0581;
mod e0582;
mod e0597;
mod e0621;
mod e0623;
mod e0625;
mod e0626;
mod e0637;
mod e0657;
mod e0700;
mod e0716;
mod e0803;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0106::ENTRY,
    &e0195::ENTRY,
    &e0226::ENTRY,
    &e0227::ENTRY,
    &e0228::ENTRY,
    &e0261::ENTRY,
    &e0262::ENTRY,
    &e0263::ENTRY,
    &e0478::ENTRY,
    &e0482::ENTRY,
    &e0491::ENTRY,
    &e0495::ENTRY,
    &e0496::ENTRY,
    &e0515::ENTRY,
    &e0581::ENTRY,
    &e0582::ENTRY,
    &e0597::ENTRY,
    &e0621::ENTRY,
    &e0623::ENTRY,
    &e0625::ENTRY,
    &e0626::ENTRY,
    &e0637::ENTRY,
    &e0657::ENTRY,
    &e0700::ENTRY,
    &e0716::ENTRY,
    &e0803::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
