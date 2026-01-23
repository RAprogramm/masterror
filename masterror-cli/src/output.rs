// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Terminal output formatting for errors.

use owo_colors::OwoColorize;

#[allow(unused_imports)]
use crate::sections;
use crate::{
    errors::{ErrorEntry, ErrorRegistry},
    options::DisplayOptions,
    parser::CargoMessage
};

const SEPARATOR: &str = "--- masterror ----------------------------------------";
const SEPARATOR_END: &str = "------------------------------------------------------";

/// Print error with masterror explanation.
pub fn print_error(lang: &str, msg: &CargoMessage, opts: &DisplayOptions) {
    let rendered = msg.rendered_output();

    #[cfg(feature = "show-original")]
    if let Some(r) = rendered {
        print!("{}", r.trim_end());
    }

    let Some(code) = msg.error_code() else {
        #[cfg(feature = "show-original")]
        println!();
        return;
    };

    let registry = ErrorRegistry::new();
    let Some(entry) = registry.find(code) else {
        #[cfg(feature = "show-original")]
        println!();
        return;
    };

    println!();
    print_block(lang, entry, msg.error_message(), rendered, opts);
}

fn print_block(
    lang: &str,
    entry: &ErrorEntry,
    #[allow(unused_variables)] error_msg: Option<&str>,
    #[allow(unused_variables)] rendered: Option<&str>,
    opts: &DisplayOptions
) {
    if opts.colored {
        println!("{}", SEPARATOR.dimmed());
    } else {
        println!("{SEPARATOR}");
    }

    #[cfg(feature = "show-translation")]
    sections::translation::print(lang, entry.code, rendered, opts.colored);

    #[cfg(feature = "show-why")]
    {
        let label = match lang {
            "ru" => "Почему это происходит:",
            "ko" => "왜 이런 일이 발생하나요:",
            _ => "Why this happens:"
        };
        if opts.colored {
            println!("{}", label.green().bold());
        } else {
            println!("{label}");
        }
        println!("{}", entry.explanation.get(lang));
    }

    #[cfg(feature = "show-fix")]
    sections::fix::print(lang, entry.fixes, opts.colored);

    #[cfg(feature = "show-link")]
    sections::link::print(lang, entry.links, opts.colored);

    if opts.colored {
        println!("{}", SEPARATOR_END.dimmed());
    } else {
        println!("{SEPARATOR_END}");
    }
}
