// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Link section - shows documentation URLs.
#![allow(dead_code)]

use owo_colors::OwoColorize;

use crate::{knowledge::DocLink, locale::Locale};

/// Print documentation links with titles.
pub fn print(locale: &Locale, links: &[DocLink], colored: bool) {
    if links.is_empty() {
        return;
    }

    let label = locale.get("label-link");
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
