// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Terminal output formatting for errors.

use masterror_knowledge::{ErrorEntry, ErrorRegistry, Lang, UiMsg};
use owo_colors::OwoColorize;

use crate::{options::DisplayOptions, parser::CargoMessage, sections};

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
    if opts.colored {
        println!("{}", SEPARATOR.dimmed());
    } else {
        println!("{SEPARATOR}");
    }

    if opts.show_translation {
        sections::translation::print(lang, rendered, opts.colored);
    }

    if opts.show_why {
        let label = UiMsg::LabelWhy.get(lang);
        if opts.colored {
            println!("{}", label.green().bold());
        } else {
            println!("{label}");
        }
        println!("{}", entry.explanation.get(lang.code()));
    }

    if opts.show_fix {
        sections::fix::print(lang, entry.fixes, opts.colored);
    }

    if opts.show_links {
        sections::link::print(lang, entry.links, opts.colored);
    }

    if opts.colored {
        println!("{}", SEPARATOR_END.dimmed());
    } else {
        println!("{SEPARATOR_END}");
    }
}
