// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Translation section - shows full translated compiler error.
#![allow(dead_code)]

use owo_colors::OwoColorize;

use crate::locale::Locale;

/// Print full translated copy of compiler error.
pub fn print(lang: &str, _error_code: &str, rendered: Option<&str>, colored: bool) {
    // Only show translation for non-English languages
    if lang == "en" {
        return;
    }

    let Some(rendered) = rendered else {
        return;
    };

    // Create locale to use its translation capability
    let locale = Locale::new(lang);
    if !locale.has_translation() {
        return;
    }

    let translated = locale.translate_rendered(rendered);

    let label = match lang {
        "ru" => "Перевод:",
        "ko" => "번역:",
        _ => "Translation:"
    };

    if colored {
        println!("{}", label.cyan().bold());
    } else {
        println!("{label}");
    }

    for line in translated.lines() {
        println!("  {line}");
    }
}
