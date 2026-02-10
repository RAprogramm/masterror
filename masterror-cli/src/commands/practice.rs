// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Practice command - show best practices from RustManifest.

use masterror_knowledge::{Lang, PracticeCategory, PracticeRegistry};

use super::explain::print_practice;
use crate::{
    error::{AppError, Result},
    options::DisplayOptions,
    output::{print_category_header, print_code_title, print_title}
};

/// List all best practices or filter by category.
pub fn list(lang: Lang, category: Option<&str>, opts: &DisplayOptions) -> Result<()> {
    let registry = PracticeRegistry::new();

    println!();
    print_title("RustManifest Best Practices", opts.colored);
    println!();

    let practices: Vec<_> = if let Some(cat) = category {
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

    if practices.is_empty() {
        println!("  No practices found.");
        return Ok(());
    }

    let mut sorted = practices;
    sorted.sort_by_key(|p| p.code);

    let mut current_cat: Option<PracticeCategory> = None;
    for practice in &sorted {
        if current_cat != Some(practice.category) {
            current_cat = Some(practice.category);
            println!();
            print_category_header(practice.category.name(lang.code()), opts.colored);
            println!();
        }
        print_code_title(practice.code, practice.title.get(lang.code()), opts.colored);
    }

    println!();
    println!("Total: {} practices", sorted.len());
    println!();
    println!("Use `masterror practice <CODE>` to see details.");
    println!();

    Ok(())
}

/// Show a specific best practice.
pub fn show(lang: Lang, code: &str, opts: &DisplayOptions) -> Result<()> {
    let registry = PracticeRegistry::new();

    let Some(practice) = registry.find(code) else {
        return Err(AppError::UnknownPracticeCode {
            code: code.to_string()
        });
    };

    print_practice(lang, practice, opts);
    Ok(())
}

fn parse_category(s: &str) -> Option<PracticeCategory> {
    match s.to_lowercase().as_str() {
        "error-handling" | "error_handling" | "errorhandling" | "errors" => {
            Some(PracticeCategory::ErrorHandling)
        }
        "performance" | "perf" => Some(PracticeCategory::Performance),
        "naming" | "names" => Some(PracticeCategory::Naming),
        "documentation" | "docs" | "doc" => Some(PracticeCategory::Documentation),
        "design" | "architecture" | "arch" => Some(PracticeCategory::Design),
        "testing" | "tests" | "test" => Some(PracticeCategory::Testing),
        "security" | "sec" => Some(PracticeCategory::Security),
        _ => None
    }
}
