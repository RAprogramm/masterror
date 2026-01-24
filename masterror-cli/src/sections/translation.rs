// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Translation section - shows full translated compiler error.

use masterror_knowledge::{Lang, UiMsg, phrases::translate_rendered};
use owo_colors::OwoColorize;

/// Print full translated copy of compiler error.
pub fn print(lang: Lang, rendered: Option<&str>, colored: bool) {
    if matches!(lang, Lang::En) {
        return;
    }

    let Some(rendered) = rendered else {
        return;
    };

    let translated = translate_rendered(rendered, lang);

    let label = UiMsg::LabelTranslation.get(lang);

    if colored {
        println!("{}", label.cyan().bold());
    } else {
        println!("{label}");
    }

    for line in translated.lines() {
        println!("  {line}");
    }
}
