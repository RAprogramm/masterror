// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Link section - shows documentation URLs.

use masterror_knowledge::{DocLink, Lang, UiMsg};
use owo_colors::OwoColorize;

/// Print documentation links with titles.
pub fn print(lang: Lang, links: &[DocLink], colored: bool) {
    if links.is_empty() {
        return;
    }

    let label = UiMsg::LabelLink.get(lang);

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
