// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Syntax and pattern related errors.

mod e0062;
mod e0063;
mod e0067;
mod e0081;
mod e0084;
mod e0091;
mod e0121;
mod e0124;
mod e0128;
mod e0131;
mod e0133;
mod e0152;
mod e0178;
mod e0229;
mod e0230;
mod e0231;
mod e0232;
mod e0264;
mod e0267;
mod e0268;
mod e0297;
mod e0408;
mod e0409;
mod e0415;
mod e0416;
mod e0519;
mod e0569;
mod e0571;
mod e0572;
mod e0579;
mod e0580;
mod e0586;
mod e0590;
mod e0627;
mod e0628;
mod e0634;
mod e0646;
mod e0648;
mod e0670;
mod e0695;
mod e0696;
mod e0697;
mod e0703;
mod e0704;
mod e0705;
mod e0742;
mod e0748;
mod e0753;
mod e0758;
mod e0762;
mod e0763;
mod e0765;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0062::ENTRY,
    &e0063::ENTRY,
    &e0067::ENTRY,
    &e0081::ENTRY,
    &e0084::ENTRY,
    &e0091::ENTRY,
    &e0121::ENTRY,
    &e0124::ENTRY,
    &e0128::ENTRY,
    &e0131::ENTRY,
    &e0133::ENTRY,
    &e0152::ENTRY,
    &e0178::ENTRY,
    &e0229::ENTRY,
    &e0230::ENTRY,
    &e0231::ENTRY,
    &e0232::ENTRY,
    &e0264::ENTRY,
    &e0267::ENTRY,
    &e0268::ENTRY,
    &e0297::ENTRY,
    &e0408::ENTRY,
    &e0409::ENTRY,
    &e0415::ENTRY,
    &e0416::ENTRY,
    &e0519::ENTRY,
    &e0569::ENTRY,
    &e0571::ENTRY,
    &e0572::ENTRY,
    &e0579::ENTRY,
    &e0580::ENTRY,
    &e0586::ENTRY,
    &e0590::ENTRY,
    &e0627::ENTRY,
    &e0628::ENTRY,
    &e0634::ENTRY,
    &e0646::ENTRY,
    &e0648::ENTRY,
    &e0670::ENTRY,
    &e0695::ENTRY,
    &e0696::ENTRY,
    &e0697::ENTRY,
    &e0703::ENTRY,
    &e0704::ENTRY,
    &e0705::ENTRY,
    &e0742::ENTRY,
    &e0748::ENTRY,
    &e0753::ENTRY,
    &e0758::ENTRY,
    &e0762::ENTRY,
    &e0763::ENTRY,
    &e0765::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
