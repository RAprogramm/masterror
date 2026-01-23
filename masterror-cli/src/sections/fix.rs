// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Fix section - shows fix suggestions with code examples.
#![allow(dead_code)]

use owo_colors::OwoColorize;

use crate::errors::FixSuggestion;

/// Print fix suggestions with code examples.
pub fn print(lang: &str, fixes: &[FixSuggestion], colored: bool) {
    if fixes.is_empty() {
        return;
    }

    let label = match lang {
        "ru" => "Как исправить:",
        "ko" => "해결 방법:",
        _ => "How to fix:"
    };

    if colored {
        println!("{}", label.green().bold());
    } else {
        println!("{label}");
    }

    for (i, fix) in fixes.iter().enumerate() {
        let desc = fix.description.get(lang);
        if colored {
            println!("  {}. {}", (i + 1).to_string().cyan(), desc);
            println!("     {}", fix.code.dimmed());
        } else {
            println!("  {}. {}", i + 1, desc);
            println!("     {}", fix.code);
        }
    }
}
