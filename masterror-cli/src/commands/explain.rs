// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Explain command - explain a specific error code.

use owo_colors::OwoColorize;

#[allow(unused_imports)]
use crate::sections;
use crate::{
    knowledge::{self, ErrorEntry},
    locale::Locale,
    options::DisplayOptions
};

/// Explain a specific error code.
pub fn run(
    locale: &Locale,
    code: &str,
    opts: &DisplayOptions
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(entry) = knowledge::find(code) else {
        eprintln!("Unknown error code: {code}");
        eprintln!("Run `masterror list` to see available codes.");
        std::process::exit(1);
    };

    print_explain(locale, entry, opts);
    Ok(())
}

fn print_explain(locale: &Locale, entry: &ErrorEntry, opts: &DisplayOptions) {
    println!();

    // Title
    let title = locale.get(entry.title_key);
    if opts.colored {
        println!("{} - {}", entry.code.yellow().bold(), title.bold());
    } else {
        println!("{} - {title}", entry.code);
    }

    // Category
    let cat_key = format!("category-{}", entry.category);
    let category = locale.get(&cat_key);
    if opts.colored {
        println!("Category: {}", category.dimmed());
    } else {
        println!("Category: {category}");
    }

    #[cfg(feature = "show-why")]
    {
        println!();
        sections::why::print(locale, entry.explanation_key, opts.colored);
    }

    #[cfg(feature = "show-fix")]
    if !entry.fixes.is_empty() {
        println!();
        sections::fix::print(locale, entry.fixes, opts.colored);
    }

    #[cfg(feature = "show-link")]
    if !entry.links.is_empty() {
        println!();
        sections::link::print(locale, entry.links, opts.colored);
    }

    println!();
}
