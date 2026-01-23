// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! List command - list all known error codes.

use owo_colors::OwoColorize;

use crate::{
    errors::{Category, ErrorRegistry},
    options::DisplayOptions
};

/// List all known error codes.
pub fn run(
    lang: &str,
    category: Option<&str>,
    opts: &DisplayOptions
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = ErrorRegistry::new();

    println!();
    if opts.colored {
        println!("{}", "Known Rust Compiler Errors".bold());
    } else {
        println!("Known Rust Compiler Errors");
    }
    println!();

    let mut entries: Vec<_> = if let Some(cat) = category {
        let cat = parse_category(cat);
        if let Some(c) = cat {
            registry.by_category(c)
        } else {
            eprintln!("Unknown category: {}", category.unwrap_or(""));
            eprintln!("Available: ownership, borrowing, lifetimes, types, traits, resolution");
            return Ok(());
        }
    } else {
        registry.all().collect()
    };

    if entries.is_empty() {
        println!("  No errors found.");
        return Ok(());
    }

    // Sort by code
    entries.sort_by_key(|e| e.code);

    let mut current_cat: Option<Category> = None;
    for entry in &entries {
        if current_cat != Some(entry.category) {
            current_cat = Some(entry.category);
            println!();
            let cat_name = entry.category.name(lang);
            if opts.colored {
                println!("  {}", cat_name.yellow().bold());
            } else {
                println!("  {cat_name}");
            }
            println!();
        }

        let title = entry.title.get(lang);
        if opts.colored {
            println!("    {} - {title}", entry.code.cyan());
        } else {
            println!("    {} - {title}", entry.code);
        }
    }

    println!();
    println!("Total: {} errors", entries.len());
    println!();
    println!("Use `masterror explain <CODE>` to see details.");
    println!("Use `masterror practice` to see best practices.");
    println!();

    Ok(())
}

fn parse_category(s: &str) -> Option<Category> {
    match s.to_lowercase().as_str() {
        "ownership" | "own" => Some(Category::Ownership),
        "borrowing" | "borrow" => Some(Category::Borrowing),
        "lifetimes" | "lifetime" | "life" => Some(Category::Lifetimes),
        "types" | "type" => Some(Category::Types),
        "traits" | "trait" => Some(Category::Traits),
        "resolution" | "resolve" | "names" => Some(Category::Resolution),
        _ => None
    }
}
