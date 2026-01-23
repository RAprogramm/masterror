// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Why section - explains the error cause in detail.
#![allow(dead_code)]

use owo_colors::OwoColorize;

use crate::locale::Locale;

/// Print detailed explanation of the error.
pub fn print(locale: &Locale, explanation_key: &str, colored: bool) {
    let label = locale.get("label-why");
    let explanation = locale.get(explanation_key);

    if colored {
        println!("{}", label.yellow().bold());
    } else {
        println!("{label}");
    }

    for line in explanation.lines() {
        println!("  {line}");
    }
}

/// Print explanation with indent (for explain command).
pub fn print_indented(locale: &Locale, explanation_key: &str, colored: bool) {
    print(locale, explanation_key, colored);
}
