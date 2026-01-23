// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Translation section - shows full translated compiler error.
#![allow(dead_code)]

use owo_colors::OwoColorize;

use crate::locale::Locale;

/// Print full translated copy of compiler error.
pub fn print(locale: &Locale, _error_code: &str, rendered: Option<&str>, colored: bool) {
    if !locale.has_translation() {
        return;
    }

    let Some(rendered) = rendered else {
        return;
    };

    let translated = locale.translate_rendered(rendered);

    let label = locale.get("label-translation");
    if colored {
        println!("{}", label.cyan().bold());
    } else {
        println!("{label}");
    }

    for line in translated.lines() {
        println!("  {line}");
    }
}
