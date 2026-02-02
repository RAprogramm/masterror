// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Trait-related errors.

mod e0034;
mod e0038;
mod e0040;
mod e0046;
mod e0049;
mod e0050;
mod e0053;
mod e0116;
mod e0117;
mod e0118;
mod e0119;
mod e0120;
mod e0183;
mod e0184;
mod e0185;
mod e0186;
mod e0191;
mod e0200;
mod e0201;
mod e0204;
mod e0205;
mod e0206;
mod e0207;
mod e0210;
mod e0220;
mod e0221;
mod e0222;
mod e0223;
mod e0224;
mod e0225;
mod e0271;
mod e0275;
mod e0276;
mod e0282;
mod e0404;
mod e0405;
mod e0407;
mod e0437;
mod e0438;
mod e0520;
mod e0525;
mod e0567;
mod e0568;
mod e0576;
mod e0592;
mod e0593;
mod e0638;
mod e0639;
mod e0642;
mod e0665;
mod e0802;
mod e0804;
mod e0805;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0034::ENTRY,
    &e0038::ENTRY,
    &e0040::ENTRY,
    &e0046::ENTRY,
    &e0049::ENTRY,
    &e0050::ENTRY,
    &e0053::ENTRY,
    &e0116::ENTRY,
    &e0117::ENTRY,
    &e0118::ENTRY,
    &e0119::ENTRY,
    &e0120::ENTRY,
    &e0183::ENTRY,
    &e0184::ENTRY,
    &e0185::ENTRY,
    &e0186::ENTRY,
    &e0191::ENTRY,
    &e0200::ENTRY,
    &e0201::ENTRY,
    &e0204::ENTRY,
    &e0205::ENTRY,
    &e0206::ENTRY,
    &e0207::ENTRY,
    &e0210::ENTRY,
    &e0220::ENTRY,
    &e0221::ENTRY,
    &e0222::ENTRY,
    &e0223::ENTRY,
    &e0224::ENTRY,
    &e0225::ENTRY,
    &e0271::ENTRY,
    &e0275::ENTRY,
    &e0276::ENTRY,
    &e0282::ENTRY,
    &e0404::ENTRY,
    &e0405::ENTRY,
    &e0407::ENTRY,
    &e0437::ENTRY,
    &e0438::ENTRY,
    &e0520::ENTRY,
    &e0525::ENTRY,
    &e0567::ENTRY,
    &e0568::ENTRY,
    &e0576::ENTRY,
    &e0592::ENTRY,
    &e0593::ENTRY,
    &e0638::ENTRY,
    &e0639::ENTRY,
    &e0642::ENTRY,
    &e0665::ENTRY,
    &e0802::ENTRY,
    &e0804::ENTRY,
    &e0805::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
