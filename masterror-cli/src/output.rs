// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Terminal output formatting for errors.

use owo_colors::OwoColorize;

#[allow(unused_imports)]
use crate::sections;
use crate::{
    knowledge::{self, ErrorEntry},
    locale::Locale,
    options::DisplayOptions,
    parser::CargoMessage
};

const SEPARATOR: &str = "--- masterror ----------------------------------------";
const SEPARATOR_END: &str = "------------------------------------------------------";

/// Print error with masterror explanation.
pub fn print_error(locale: &Locale, msg: &CargoMessage, opts: &DisplayOptions) {
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
    let Some(entry) = knowledge::find(code) else {
        #[cfg(feature = "show-original")]
        println!();
        return;
    };

    println!();
    print_block(locale, entry, msg.error_message(), rendered, opts);
}

fn print_block(
    locale: &Locale,
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
    sections::translation::print(locale, entry.code, rendered, opts.colored);

    #[cfg(feature = "show-why")]
    sections::why::print(locale, entry.explanation_key, opts.colored);

    #[cfg(feature = "show-fix")]
    sections::fix::print(locale, entry.fixes, opts.colored);

    #[cfg(feature = "show-link")]
    sections::link::print(locale, entry.links, opts.colored);

    if opts.colored {
        println!("{}", SEPARATOR_END.dimmed());
    } else {
        println!("{SEPARATOR_END}");
    }
}
