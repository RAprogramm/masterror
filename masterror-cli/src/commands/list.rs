// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! List command - list all known error codes.

use masterror_knowledge::{Category, ErrorRegistry, Lang};

use crate::{
    error::{AppError, Result},
    options::DisplayOptions,
    output::{print_category_header, print_code_title, print_title}
};

/// List all known error codes.
pub fn run(lang: Lang, category: Option<&str>, opts: &DisplayOptions) -> Result<()> {
    let registry = ErrorRegistry::new();

    println!();
    print_title("Known Rust Compiler Errors", opts.colored);
    println!();

    let mut entries: Vec<_> = if let Some(cat) = category {
        let cat = parse_category(cat);
        if let Some(c) = cat {
            registry.by_category(c)
        } else {
            return Err(AppError::InvalidCategory {
                name: category.unwrap_or("").to_string()
            });
        }
    } else {
        registry.all().collect()
    };

    if entries.is_empty() {
        println!("  No errors found.");
        return Ok(());
    }

    entries.sort_by_key(|e| e.code);

    let mut current_cat: Option<Category> = None;
    for entry in &entries {
        if current_cat != Some(entry.category) {
            current_cat = Some(entry.category);
            println!();
            print_category_header(entry.category.name(lang.code()), opts.colored);
            println!();
        }
        print_code_title(entry.code, entry.title.get(lang.code()), opts.colored);
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
