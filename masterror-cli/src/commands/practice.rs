// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Practice command - show best practices from RustManifest.

use owo_colors::OwoColorize;

use crate::{
    errors::raprogramm::{BestPractice, PracticeCategory, PracticeRegistry},
    options::DisplayOptions
};

/// List all best practices or filter by category.
pub fn list(
    lang: &str,
    category: Option<&str>,
    opts: &DisplayOptions
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = PracticeRegistry::new();

    println!();
    if opts.colored {
        println!("{}", "RustManifest Best Practices".bold());
    } else {
        println!("RustManifest Best Practices");
    }
    println!();

    let practices: Vec<_> = if let Some(cat) = category {
        let cat = parse_category(cat);
        if let Some(c) = cat {
            registry.by_category(c)
        } else {
            eprintln!("Unknown category: {}", category.unwrap_or(""));
            eprintln!(
                "Available: error-handling, performance, naming, documentation, design, testing, security"
            );
            return Ok(());
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
            if opts.colored {
                println!("  {}", practice.category.name(lang).yellow().bold());
            } else {
                println!("  {}", practice.category.name(lang));
            }
            println!();
        }

        let title = practice.title.get(lang);
        if opts.colored {
            println!("    {} - {title}", practice.code.cyan());
        } else {
            println!("    {} - {title}", practice.code);
        }
    }

    println!();
    println!("Total: {} practices", sorted.len());
    println!();
    println!("Use `masterror practice <CODE>` to see details.");
    println!();

    Ok(())
}

/// Show a specific best practice.
pub fn show(
    lang: &str,
    code: &str,
    opts: &DisplayOptions
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = PracticeRegistry::new();

    let Some(practice) = registry.find(code) else {
        eprintln!("Unknown practice code: {code}");
        eprintln!("Run `masterror practice` to see available codes.");
        std::process::exit(1);
    };

    print_practice(lang, practice, opts);
    Ok(())
}

fn print_practice(lang: &str, practice: &BestPractice, opts: &DisplayOptions) {
    println!();

    // Title
    let title = practice.title.get(lang);
    if opts.colored {
        println!("{} - {}", practice.code.yellow().bold(), title.bold());
    } else {
        println!("{} - {title}", practice.code);
    }

    // Category
    let category = practice.category.name(lang);
    if opts.colored {
        println!("Category: {}", category.dimmed());
    } else {
        println!("Category: {category}");
    }

    // Explanation
    println!();
    let why_label = match lang {
        "ru" => "Почему это важно:",
        "ko" => "왜 중요한가:",
        _ => "Why this matters:"
    };
    if opts.colored {
        println!("{}", why_label.green().bold());
    } else {
        println!("{why_label}");
    }
    println!("{}", practice.explanation.get(lang));

    // How to apply
    println!();
    let how_label = match lang {
        "ru" => "Как применять:",
        "ko" => "적용 방법:",
        _ => "How to apply:"
    };
    if opts.colored {
        println!("{}", how_label.green().bold());
    } else {
        println!("{how_label}");
    }

    // Bad example
    println!();
    let avoid_label = match lang {
        "ru" => "Избегайте",
        "ko" => "피하세요",
        _ => "Avoid"
    };
    if opts.colored {
        println!("{}. {}", "1".cyan(), avoid_label.red());
    } else {
        println!("1. {avoid_label}");
    }
    println!("```rust");
    println!("{}", practice.bad_example);
    println!("```");

    // Good example
    println!();
    let prefer_label = match lang {
        "ru" => "Предпочитайте",
        "ko" => "선호하세요",
        _ => "Prefer"
    };
    if opts.colored {
        println!("{}. {}", "2".cyan(), prefer_label.green());
    } else {
        println!("2. {prefer_label}");
    }
    println!("```rust");
    println!("{}", practice.good_example);
    println!("```");

    // Source
    println!();
    let learn_label = match lang {
        "ru" => "Подробнее:",
        "ko" => "더 알아보기:",
        _ => "Learn more:"
    };
    if opts.colored {
        println!("{}", learn_label.cyan().bold());
    } else {
        println!("{learn_label}");
    }
    if opts.colored {
        println!("  - RustManifest {}", practice.source.dimmed());
    } else {
        println!("  - RustManifest {}", practice.source);
    }

    println!();
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
