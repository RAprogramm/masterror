// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Terminal output formatting for errors.

use masterror_knowledge::{ErrorEntry, ErrorRegistry, Lang, UiMsg};
use owo_colors::OwoColorize;

use crate::{options::DisplayOptions, parser::CargoMessage, sections};

// ─────────────────────────────────────────────────────────────────────────────
// Colored output helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Print text as bold title.
pub fn print_title(text: &str, colored: bool) {
    if colored {
        println!("{}", text.bold());
    } else {
        println!("{text}");
    }
}

/// Print category header (yellow bold).
pub fn print_category_header(text: &str, colored: bool) {
    if colored {
        println!("  {}", text.yellow().bold());
    } else {
        println!("  {text}");
    }
}

/// Print code with title (code in cyan).
pub fn print_code_title(code: &str, title: &str, colored: bool) {
    if colored {
        println!("    {} - {title}", code.cyan());
    } else {
        println!("    {code} - {title}");
    }
}

/// Print section label (green bold).
pub fn print_label(label: &str, colored: bool) {
    if colored {
        println!("{}", label.green().bold());
    } else {
        println!("{label}");
    }
}

/// Print dimmed text.
pub fn print_dimmed(text: &str, colored: bool) {
    if colored {
        println!("{}", text.dimmed());
    } else {
        println!("{text}");
    }
}

const SEPARATOR: &str = "--- masterror ----------------------------------------";
const SEPARATOR_END: &str = "------------------------------------------------------";

/// Print error with masterror explanation.
pub fn print_error(lang: Lang, msg: &CargoMessage, opts: &DisplayOptions) {
    let rendered = msg.rendered_output();

    if opts.show_original
        && let Some(r) = rendered
    {
        print!("{}", r.trim_end());
    }

    let Some(code) = msg.error_code() else {
        if opts.show_original {
            println!();
        }
        return;
    };

    let registry = ErrorRegistry::new();
    let Some(entry) = registry.find(code) else {
        if opts.show_original {
            println!();
        }
        return;
    };

    println!();
    print_block(lang, entry, rendered, opts);
}

fn print_block(lang: Lang, entry: &ErrorEntry, rendered: Option<&str>, opts: &DisplayOptions) {
    print_dimmed(SEPARATOR, opts.colored);

    if opts.show_translation {
        sections::translation::print(lang, rendered, opts.colored);
    }

    if opts.show_why {
        print_label(UiMsg::LabelWhy.get(lang), opts.colored);
        println!("{}", entry.explanation.get(lang.code()));
    }

    if opts.show_fix {
        sections::fix::print(lang, entry.fixes, opts.colored);
    }

    if opts.show_links {
        sections::link::print(lang, entry.links, opts.colored);
    }

    print_dimmed(SEPARATOR_END, opts.colored);
}
