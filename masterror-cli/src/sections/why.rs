// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Why section - explains the error cause in detail.
#![allow(dead_code)]

use owo_colors::OwoColorize;

/// Print detailed explanation of the error.
pub fn print(lang: &str, explanation: &str, colored: bool) {
    let label = match lang {
        "ru" => "Почему это происходит:",
        "ko" => "왜 이런 일이 발생하나요:",
        _ => "Why this happens:"
    };

    if colored {
        println!("{}", label.yellow().bold());
    } else {
        println!("{label}");
    }

    for line in explanation.lines() {
        println!("  {line}");
    }
}
