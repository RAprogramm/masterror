// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! masterror CLI - Rust compiler error explainer.

mod commands;
mod config;
mod error;
mod options;
mod output;
mod parser;
mod sections;

use clap::{Parser, Subcommand};
use masterror_knowledge::Lang;

use crate::{config::Config, options::DisplayOptions};

#[derive(Parser)]
#[command(name = "masterror")]
#[command(author, version, about = "Rust compiler error explainer")]
struct Cli {
    /// Language for explanations (en, ru, ko)
    #[arg(short, long, env = "MASTERROR_LANG")]
    lang: Option<String>,

    /// Disable colored output
    #[arg(long, env = "NO_COLOR")]
    no_color: bool,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize configuration file
    Init,
    /// Run cargo check and explain errors
    #[command(visible_alias = "c")]
    Check {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>
    },
    /// Explain a specific error code (E0382) or best practice (RA001)
    #[command(visible_alias = "e")]
    Explain { code: String },
    /// List all known error codes
    #[command(visible_alias = "l")]
    List {
        #[arg(short, long)]
        category: Option<String>
    },
    /// Show RustManifest best practices
    #[command(visible_alias = "p")]
    Practice {
        /// Practice code (RA001-RA015) or empty for list
        code:     Option<String>,
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli = if args.get(1).is_some_and(|a| a == "masterror") {
        Cli::parse_from(
            args.into_iter()
                .enumerate()
                .filter_map(|(i, a)| if i == 1 { None } else { Some(a) })
        )
    } else {
        Cli::parse()
    };

    // Check for first run and run setup if needed (except for init command)
    if !matches!(cli.command, Commands::Init)
        && let Err(e) = commands::init::check_first_run(true)
    {
        eprintln!("Setup failed: {e}");
        std::process::exit(1);
    }

    let config = Config::load().unwrap_or_default();
    let lang_str = cli.lang.as_deref().unwrap_or(&config.general.lang);
    let lang = Lang::from_code(lang_str);
    let colored = if cli.no_color {
        false
    } else {
        config.general.colored
    };

    let opts = DisplayOptions {
        colored,
        show_translation: config.display.translation,
        show_why: config.display.why,
        show_fix: config.display.fix,
        show_links: config.display.links,
        show_original: config.display.original
    };

    let result = match &cli.command {
        Commands::Init => commands::init(lang, opts.colored),
        Commands::Check {
            args
        } => commands::check(lang, args, &opts),
        Commands::Explain {
            code
        } => commands::explain(lang, code, &opts),
        Commands::List {
            category
        } => commands::list(lang, category.as_deref(), &opts),
        Commands::Practice {
            code,
            category
        } => {
            if let Some(c) = code {
                commands::practice::show(lang, c, &opts)
            } else {
                commands::practice::list(lang, category.as_deref(), &opts)
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
