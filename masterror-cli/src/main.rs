// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! masterror CLI - Rust compiler error explainer.

mod commands;
mod knowledge;
mod locale;
mod options;
mod output;
mod parser;
mod sections;

use clap::{Parser, Subcommand};

use crate::{locale::Locale, options::DisplayOptions};

#[derive(Parser)]
#[command(name = "masterror")]
#[command(author, version, about = "Rust compiler error explainer")]
struct Cli {
    /// Language for explanations (en, ru, ko)
    #[arg(short, long, env = "MASTERROR_LANG", default_value = "en")]
    lang: String,

    /// Disable colored output
    #[arg(long, env = "NO_COLOR")]
    no_color: bool,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Run cargo check and explain errors
    Check {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>
    },
    /// Explain a specific error code
    Explain { code: String },
    /// List all known error codes
    List {
        #[arg(short, long)]
        category: Option<String>
    }
}

fn main() {
    let cli = Cli::parse();
    let locale = Locale::new(&cli.lang);
    let opts = DisplayOptions {
        colored: !cli.no_color
    };

    let result = match &cli.command {
        Commands::Check {
            args
        } => commands::check(&locale, args, &opts),
        Commands::Explain {
            code
        } => commands::explain(&locale, code, &opts),
        Commands::List {
            category
        } => commands::list(&locale, category.as_deref(), &opts)
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
