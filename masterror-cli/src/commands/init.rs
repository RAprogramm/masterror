// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Init command - create configuration file interactively.

use std::io::{self, Write};

use masterror_knowledge::{Lang, UiMsg};
use owo_colors::OwoColorize;

use crate::{
    config::{Config, DisplayConfig, GeneralConfig},
    error::Result
};

/// Run init command - create configuration interactively.
pub fn run(_lang: Lang, colored: bool) -> Result<()> {
    println!();
    print_welcome(colored);
    println!();

    // First ask language - this affects all subsequent prompts
    let lang_value = prompt_language(colored)?;
    let lang = masterror_knowledge::Lang::from_code(&lang_value);

    let color_value = prompt_colors(lang, colored)?;
    let display = prompt_display(lang, colored)?;
    let save_location = prompt_save_location(lang, colored)?;

    let config = Config {
        general: GeneralConfig {
            lang:    lang_value,
            colored: color_value
        },
        display,
        aliases: default_aliases()
    };

    let saved_path = match save_location {
        SaveLocation::Global => {
            config.save()?;
            Config::path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "~/.config/masterror/config.toml".to_string())
        }
        SaveLocation::Local => {
            let path = config.save_local()?;
            path.display().to_string()
        }
    };

    println!();
    print_success(lang, colored, &saved_path);
    print_tips(lang, colored, &save_location);

    Ok(())
}

/// Check if first run and prompt for setup.
pub fn check_first_run(colored: bool) -> Result<bool> {
    if !Config::is_first_run() {
        return Ok(false);
    }

    println!();
    if colored {
        println!(
            "{}",
            "Welcome to masterror! Let's set up your preferences.".cyan()
        );
        println!(
            "{}",
            "(This only happens once. Run `masterror init` to reconfigure later.)".dimmed()
        );
    } else {
        println!("Welcome to masterror! Let's set up your preferences.");
        println!("(This only happens once. Run `masterror init` to reconfigure later.)");
    }
    println!();

    run(Lang::En, colored)?;
    Ok(true)
}

#[derive(Clone, Copy)]
enum SaveLocation {
    Global,
    Local
}

fn print_welcome(colored: bool) {
    let title = "masterror - Rust compiler error explainer";
    let subtitle = "Let's configure your preferences";

    if colored {
        println!("{}", title.bold().cyan());
        println!("{}", subtitle.dimmed());
    } else {
        println!("{title}");
        println!("{subtitle}");
    }
}

fn prompt_language(colored: bool) -> Result<String> {
    println!();
    if colored {
        println!(
            "{}",
            "Select your language / Выберите язык / 언어 선택".bold()
        );
        println!("  {} - English", "en".cyan());
        println!("  {} - Русский", "ru".cyan());
        println!("  {} - 한국어", "ko".cyan());
    } else {
        println!("Select your language / Выберите язык / 언어 선택");
        println!("  en - English");
        println!("  ru - Русский");
        println!("  ko - 한국어");
    }

    if colored {
        print!("{} [en/ru/ko] ({}): ", "Choice".bold(), "en".dimmed());
    } else {
        print!("Choice [en/ru/ko] (en): ");
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim().to_lowercase();

    Ok(
        if choice.is_empty() || !["en", "ru", "ko"].contains(&choice.as_str()) {
            "en".to_string()
        } else {
            choice
        }
    )
}

fn prompt_colors(lang: Lang, colored: bool) -> Result<bool> {
    let prompt = UiMsg::InitColorPrompt.get(lang);

    if colored {
        print!("{} [y/n] ({}): ", prompt.bold(), "y".dimmed());
    } else {
        print!("{prompt} [y/n] (y): ");
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(!matches!(input.trim().to_lowercase().as_str(), "n" | "no"))
}

fn prompt_display(lang: Lang, colored: bool) -> Result<DisplayConfig> {
    let prompt = UiMsg::InitDisplayPrompt.get(lang);
    println!();
    if colored {
        println!("{}", prompt.bold());
    } else {
        println!("{prompt}");
    }

    let translation = prompt_bool(lang, colored, UiMsg::InitShowTranslation, true)?;
    let why = prompt_bool(lang, colored, UiMsg::InitShowWhy, true)?;
    let fix = prompt_bool(lang, colored, UiMsg::InitShowFix, true)?;
    let links = prompt_bool(lang, colored, UiMsg::InitShowLinks, true)?;
    let original = prompt_bool(lang, colored, UiMsg::InitShowOriginal, false)?;

    Ok(DisplayConfig {
        translation,
        why,
        fix,
        links,
        original
    })
}

fn prompt_save_location(lang: Lang, colored: bool) -> Result<SaveLocation> {
    println!();
    let global_label = UiMsg::InitSaveGlobal.get(lang);
    let local_label = UiMsg::InitSaveLocal.get(lang);

    if colored {
        println!("{}", UiMsg::InitSavePrompt.get(lang).bold());
        println!("  {} - {}", "1".cyan(), global_label);
        println!("  {} - {}", "2".cyan(), local_label);
    } else {
        println!("{}", UiMsg::InitSavePrompt.get(lang));
        println!("  1 - {global_label}");
        println!("  2 - {local_label}");
    }

    if colored {
        print!("{} [1/2] ({}): ", "Choice".bold(), "1".dimmed());
    } else {
        print!("Choice [1/2] (1): ");
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(match input.trim() {
        "2" => SaveLocation::Local,
        _ => SaveLocation::Global
    })
}

fn prompt_bool(lang: Lang, colored: bool, msg: UiMsg, default: bool) -> Result<bool> {
    let label = msg.get(lang);
    let default_str = if default { "y" } else { "n" };

    if colored {
        print!("  {} [y/n] ({}): ", label, default_str.dimmed());
    } else {
        print!("  {label} [y/n] ({default_str}): ");
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();

    Ok(if trimmed.is_empty() {
        default
    } else {
        matches!(trimmed.as_str(), "y" | "yes" | "д" | "да")
    })
}

fn print_success(lang: Lang, colored: bool, path: &str) {
    let msg = UiMsg::InitSuccess.get(lang);

    if colored {
        println!("{} {}", msg.green(), path.dimmed());
    } else {
        println!("{msg} {path}");
    }
}

fn print_tips(lang: Lang, colored: bool, location: &SaveLocation) {
    println!();
    let tip = UiMsg::InitTip.get(lang);

    if colored {
        println!("{}", tip.dimmed());
    } else {
        println!("{tip}");
    }

    match location {
        SaveLocation::Global => {
            let global_path = Config::path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "~/.config/masterror/config.toml".to_string());
            if colored {
                println!("  {} {}", "Global:".dimmed(), global_path.dimmed());
            } else {
                println!("  Global: {global_path}");
            }
        }
        SaveLocation::Local => {
            if colored {
                println!("  {} .masterror.toml", "Local:".dimmed());
            } else {
                println!("  Local: .masterror.toml");
            }
        }
    }

    println!();
    let usage = UiMsg::InitUsage.get(lang);
    if colored {
        println!("{}", usage.cyan());
        println!("  {} cargo masterror check", "$".dimmed());
        println!("  {} masterror explain E0382", "$".dimmed());
    } else {
        println!("{usage}");
        println!("  $ cargo masterror check");
        println!("  $ masterror explain E0382");
    }
    println!();
}

fn default_aliases() -> std::collections::HashMap<String, String> {
    let mut aliases = std::collections::HashMap::new();
    aliases.insert("c".to_string(), "check".to_string());
    aliases.insert("e".to_string(), "explain".to_string());
    aliases.insert("l".to_string(), "list".to_string());
    aliases.insert("p".to_string(), "practice".to_string());
    aliases
}
