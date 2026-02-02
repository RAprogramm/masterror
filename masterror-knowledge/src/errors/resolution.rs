// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Name resolution errors.

mod e0251;
mod e0252;
mod e0253;
mod e0254;
mod e0255;
mod e0256;
mod e0259;
mod e0260;
mod e0411;
mod e0412;
mod e0422;
mod e0423;
mod e0424;
mod e0425;
mod e0426;
mod e0428;
mod e0429;
mod e0430;
mod e0431;
mod e0432;
mod e0433;
mod e0434;
mod e0435;
mod e0436;
mod e0530;
mod e0531;
mod e0532;
mod e0533;
mod e0577;
mod e0583;
mod e0601;
mod e0602;
mod e0603;
mod e0615;
mod e0616;
mod e0624;
mod e0659;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0251::ENTRY,
    &e0252::ENTRY,
    &e0253::ENTRY,
    &e0254::ENTRY,
    &e0255::ENTRY,
    &e0256::ENTRY,
    &e0259::ENTRY,
    &e0260::ENTRY,
    &e0411::ENTRY,
    &e0412::ENTRY,
    &e0422::ENTRY,
    &e0423::ENTRY,
    &e0424::ENTRY,
    &e0425::ENTRY,
    &e0426::ENTRY,
    &e0428::ENTRY,
    &e0429::ENTRY,
    &e0430::ENTRY,
    &e0431::ENTRY,
    &e0432::ENTRY,
    &e0433::ENTRY,
    &e0434::ENTRY,
    &e0435::ENTRY,
    &e0436::ENTRY,
    &e0530::ENTRY,
    &e0531::ENTRY,
    &e0532::ENTRY,
    &e0533::ENTRY,
    &e0577::ENTRY,
    &e0583::ENTRY,
    &e0601::ENTRY,
    &e0602::ENTRY,
    &e0603::ENTRY,
    &e0615::ENTRY,
    &e0616::ENTRY,
    &e0624::ENTRY,
    &e0659::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
