// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Type-related errors.

mod e0054;
mod e0055;
mod e0057;
mod e0061;
mod e0069;
mod e0070;
mod e0071;
mod e0072;
mod e0161;
mod e0203;
mod e0208;
mod e0211;
mod e0212;
mod e0214;
mod e0243;
mod e0244;
mod e0277;
mod e0281;
mod e0283;
mod e0284;
mod e0308;
mod e0391;
mod e0472;
mod e0476;
mod e0511;
mod e0512;
mod e0516;
mod e0527;
mod e0528;
mod e0529;
mod e0559;
mod e0560;
mod e0561;
mod e0562;
mod e0573;
mod e0574;
mod e0575;
mod e0591;
mod e0599;
mod e0600;
mod e0604;
mod e0605;
mod e0606;
mod e0607;
mod e0608;
mod e0609;
mod e0610;
mod e0614;
mod e0617;
mod e0618;
mod e0620;
mod e0631;
mod e0641;
mod e0643;
mod e0644;
mod e0666;
mod e0689;
mod e0690;
mod e0691;
mod e0692;
mod e0693;
mod e0800;
mod e0801;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0054::ENTRY,
    &e0055::ENTRY,
    &e0057::ENTRY,
    &e0061::ENTRY,
    &e0069::ENTRY,
    &e0070::ENTRY,
    &e0071::ENTRY,
    &e0072::ENTRY,
    &e0161::ENTRY,
    &e0203::ENTRY,
    &e0208::ENTRY,
    &e0211::ENTRY,
    &e0212::ENTRY,
    &e0214::ENTRY,
    &e0243::ENTRY,
    &e0244::ENTRY,
    &e0277::ENTRY,
    &e0281::ENTRY,
    &e0283::ENTRY,
    &e0284::ENTRY,
    &e0308::ENTRY,
    &e0391::ENTRY,
    &e0472::ENTRY,
    &e0476::ENTRY,
    &e0511::ENTRY,
    &e0512::ENTRY,
    &e0516::ENTRY,
    &e0527::ENTRY,
    &e0528::ENTRY,
    &e0529::ENTRY,
    &e0559::ENTRY,
    &e0560::ENTRY,
    &e0561::ENTRY,
    &e0562::ENTRY,
    &e0573::ENTRY,
    &e0574::ENTRY,
    &e0575::ENTRY,
    &e0591::ENTRY,
    &e0599::ENTRY,
    &e0600::ENTRY,
    &e0604::ENTRY,
    &e0605::ENTRY,
    &e0606::ENTRY,
    &e0607::ENTRY,
    &e0608::ENTRY,
    &e0609::ENTRY,
    &e0610::ENTRY,
    &e0614::ENTRY,
    &e0617::ENTRY,
    &e0618::ENTRY,
    &e0620::ENTRY,
    &e0631::ENTRY,
    &e0641::ENTRY,
    &e0643::ENTRY,
    &e0644::ENTRY,
    &e0666::ENTRY,
    &e0689::ENTRY,
    &e0690::ENTRY,
    &e0691::ENTRY,
    &e0692::ENTRY,
    &e0693::ENTRY,
    &e0800::ENTRY,
    &e0801::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
