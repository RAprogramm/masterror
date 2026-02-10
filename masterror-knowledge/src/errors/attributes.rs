// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Attribute and linking related errors.

mod e0452;
mod e0453;
mod e0454;
mod e0455;
mod e0457;
mod e0458;
mod e0459;
mod e0466;
mod e0468;
mod e0469;
mod e0517;
mod e0518;
mod e0522;
mod e0536;
mod e0537;
mod e0538;
mod e0539;
mod e0541;
mod e0552;
mod e0554;
mod e0556;
mod e0557;
mod e0565;
mod e0566;
mod e0587;
mod e0588;
mod e0589;

use super::ErrorEntry;

static ENTRIES: &[&ErrorEntry] = &[
    &e0452::ENTRY,
    &e0453::ENTRY,
    &e0454::ENTRY,
    &e0455::ENTRY,
    &e0457::ENTRY,
    &e0458::ENTRY,
    &e0459::ENTRY,
    &e0466::ENTRY,
    &e0468::ENTRY,
    &e0469::ENTRY,
    &e0517::ENTRY,
    &e0518::ENTRY,
    &e0522::ENTRY,
    &e0536::ENTRY,
    &e0537::ENTRY,
    &e0538::ENTRY,
    &e0539::ENTRY,
    &e0541::ENTRY,
    &e0552::ENTRY,
    &e0554::ENTRY,
    &e0556::ENTRY,
    &e0557::ENTRY,
    &e0565::ENTRY,
    &e0566::ENTRY,
    &e0587::ENTRY,
    &e0588::ENTRY,
    &e0589::ENTRY
];

pub fn entries() -> &'static [&'static ErrorEntry] {
    ENTRIES
}
