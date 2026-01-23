// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Fix section - shows fix suggestions with code examples.
#![allow(dead_code)]

use owo_colors::OwoColorize;

use crate::{knowledge::FixSuggestion, locale::Locale};

/// Print fix suggestions with code examples.
pub fn print(locale: &Locale, fixes: &[FixSuggestion], colored: bool) {
    if fixes.is_empty() {
        return;
    }

    let label = locale.get("label-fix");
    if colored {
        println!("{}", label.green().bold());
    } else {
        println!("{label}");
    }

    for (i, fix) in fixes.iter().enumerate() {
        let desc = locale.get(fix.description_key);
        if colored {
            println!("  {}. {}", (i + 1).to_string().cyan(), desc);
            println!("     {}", fix.code.dimmed());
        } else {
            println!("  {}. {}", i + 1, desc);
            println!("     {}", fix.code);
        }
    }
}
