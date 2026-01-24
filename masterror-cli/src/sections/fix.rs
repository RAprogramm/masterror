// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Fix section - shows fix suggestions with code examples.

use masterror_knowledge::{FixSuggestion, Lang, UiMsg};
use owo_colors::OwoColorize;

/// Print fix suggestions with code examples.
pub fn print(lang: Lang, fixes: &[FixSuggestion], colored: bool) {
    if fixes.is_empty() {
        return;
    }

    let label = UiMsg::LabelFix.get(lang);

    if colored {
        println!("{}", label.green().bold());
    } else {
        println!("{label}");
    }

    for (i, fix) in fixes.iter().enumerate() {
        let desc = fix.description.get(lang.code());
        let num = i + 1;
        if colored {
            println!("  {}. {}", num.cyan(), desc);
            println!("     {}", fix.code.dimmed());
        } else {
            println!("  {num}. {desc}");
            println!("     {}", fix.code);
        }
    }
}
