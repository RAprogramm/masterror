// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Link section - shows documentation URLs.
#![allow(dead_code)]

use owo_colors::OwoColorize;

use crate::errors::DocLink;

/// Print documentation links with titles.
pub fn print(lang: &str, links: &[DocLink], colored: bool) {
    if links.is_empty() {
        return;
    }

    let label = match lang {
        "ru" => "Ссылки:",
        "ko" => "링크:",
        _ => "Links:"
    };

    if colored {
        println!("{}", label.blue().bold());
    } else {
        println!("{label}");
    }

    for link in links {
        if colored {
            println!("  {} {}", link.title.cyan(), link.url.underline().dimmed());
        } else {
            println!("  {} {}", link.title, link.url);
        }
    }
}
