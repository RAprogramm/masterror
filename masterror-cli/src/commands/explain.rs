// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Explain command - explain a specific error code or best practice.

use masterror_knowledge::{
    BestPractice, ErrorEntry, ErrorRegistry, Lang, PracticeRegistry, UiMsg
};
use owo_colors::OwoColorize;

use crate::{
    error::{AppError, Result},
    options::DisplayOptions
};

/// Explain a specific error code (E0382) or best practice (RA001).
pub fn run(lang: Lang, code: &str, opts: &DisplayOptions) -> Result<()> {
    let upper = code.to_uppercase();

    if upper.starts_with("RA") {
        let registry = PracticeRegistry::new();
        if let Some(practice) = registry.find(&upper) {
            print_practice(lang, practice, opts);
            return Ok(());
        }
    }

    let registry = ErrorRegistry::new();
    if let Some(entry) = registry.find(code) {
        print_error(lang, entry, opts);
        return Ok(());
    }

    Err(AppError::UnknownErrorCode {
        code: code.to_string()
    })
}

fn print_error(lang: Lang, entry: &ErrorEntry, opts: &DisplayOptions) {
    println!();

    let title = entry.title.get(lang.code());
    if opts.colored {
        println!("{} - {}", entry.code.yellow().bold(), title.bold());
    } else {
        println!("{} - {title}", entry.code);
    }

    let category = entry.category.name(lang.code());
    if opts.colored {
        println!("{}: {}", UiMsg::Category.get(lang), category.dimmed());
    } else {
        println!("{}: {category}", UiMsg::Category.get(lang));
    }

    println!();
    let why_label = UiMsg::LabelWhy.get(lang);
    if opts.colored {
        println!("{}", why_label.green().bold());
    } else {
        println!("{why_label}");
    }
    println!("{}", entry.explanation.get(lang.code()));

    if !entry.fixes.is_empty() {
        println!();
        let fix_label = UiMsg::LabelFix.get(lang);
        if opts.colored {
            println!("{}", fix_label.green().bold());
        } else {
            println!("{fix_label}");
        }
        for (i, fix) in entry.fixes.iter().enumerate() {
            println!();
            println!("{}. {}", i + 1, fix.description.get(lang.code()));
            println!("```rust");
            println!("{}", fix.code);
            println!("```");
        }
    }

    if !entry.links.is_empty() {
        println!();
        let link_label = UiMsg::LabelLink.get(lang);
        if opts.colored {
            println!("{}", link_label.cyan().bold());
        } else {
            println!("{link_label}");
        }
        for link in entry.links {
            if opts.colored {
                println!("  - {} {}", link.title, link.url.dimmed());
            } else {
                println!("  - {} {}", link.title, link.url);
            }
        }
    }

    println!();
}

/// Print best practice details.
pub fn print_practice(lang: Lang, practice: &BestPractice, opts: &DisplayOptions) {
    println!();

    let title = practice.title.get(lang.code());
    if opts.colored {
        println!("{} - {}", practice.code.yellow().bold(), title.bold());
    } else {
        println!("{} - {title}", practice.code);
    }

    let category = practice.category.name(lang.code());
    if opts.colored {
        println!("{}: {}", UiMsg::Category.get(lang), category.dimmed());
    } else {
        println!("{}: {category}", UiMsg::Category.get(lang));
    }

    println!();
    let why_label = UiMsg::LabelWhyMatters.get(lang);
    if opts.colored {
        println!("{}", why_label.green().bold());
    } else {
        println!("{why_label}");
    }
    println!("{}", practice.explanation.get(lang.code()));

    println!();
    let how_label = UiMsg::LabelHowToApply.get(lang);
    if opts.colored {
        println!("{}", how_label.green().bold());
    } else {
        println!("{how_label}");
    }

    println!();
    let avoid_label = UiMsg::LabelAvoid.get(lang);
    if opts.colored {
        println!("{}. {}", "1".cyan(), avoid_label.red());
    } else {
        println!("1. {avoid_label}");
    }
    println!("```rust");
    println!("{}", practice.bad_example);
    println!("```");

    println!();
    let prefer_label = UiMsg::LabelPrefer.get(lang);
    if opts.colored {
        println!("{}. {}", "2".cyan(), prefer_label.green());
    } else {
        println!("2. {prefer_label}");
    }
    println!("```rust");
    println!("{}", practice.good_example);
    println!("```");

    println!();
    let link_label = UiMsg::LabelLink.get(lang);
    if opts.colored {
        println!("{}", link_label.cyan().bold());
    } else {
        println!("{link_label}");
    }
    if opts.colored {
        println!("  - RustManifest {}", practice.source.dimmed());
    } else {
        println!("  - RustManifest {}", practice.source);
    }

    println!();
}
