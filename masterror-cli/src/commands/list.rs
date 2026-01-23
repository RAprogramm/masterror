// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! List command - list all known error codes.

use owo_colors::OwoColorize;

use crate::{knowledge, locale::Locale, options::DisplayOptions};

/// List all known error codes.
pub fn run(
    locale: &Locale,
    category: Option<&str>,
    opts: &DisplayOptions
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    if opts.colored {
        println!("{}", "Known Rust Compiler Errors".bold());
    } else {
        println!("Known Rust Compiler Errors");
    }
    println!();

    let entries = knowledge::entries();
    let filtered: Vec<_> = if let Some(cat) = category {
        entries
            .iter()
            .filter(|e| e.category.eq_ignore_ascii_case(cat))
            .collect()
    } else {
        entries.iter().collect()
    };

    if filtered.is_empty() {
        println!("  No errors found.");
        return Ok(());
    }

    let mut current_cat = "";
    for entry in &filtered {
        if entry.category != current_cat {
            current_cat = entry.category;
            println!();
            let cat_key = format!("category-{current_cat}");
            let cat_name = locale.get(&cat_key);
            if opts.colored {
                println!("  {}", cat_name.yellow().bold());
            } else {
                println!("  {cat_name}");
            }
            println!();
        }

        let title = locale.get(entry.title_key);
        if opts.colored {
            println!("    {} - {title}", entry.code.cyan());
        } else {
            println!("    {} - {title}", entry.code);
        }
    }

    println!();
    println!("Total: {} errors", filtered.len());
    println!();

    Ok(())
}
