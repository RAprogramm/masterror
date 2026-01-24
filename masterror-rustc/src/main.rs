// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RUSTC_WRAPPER that adds translated explanations to Rust compiler errors.
//!
//! # Usage
//!
//! Add to `.cargo/config.toml`:
//! ```toml
//! [build]
//! rustc-wrapper = "masterror-rustc"
//! ```
//!
//! Or set environment variable:
//! ```bash
//! export RUSTC_WRAPPER=masterror-rustc
//! ```
//!
//! Then use cargo as usual:
//! ```bash
//! cargo build
//! cargo run
//! cargo test
//! ```

use std::{
    env,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio, exit}
};

use masterror_knowledge::{ErrorRegistry, Lang};
use owo_colors::OwoColorize;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("masterror-rustc: no arguments provided");
        eprintln!("This is a RUSTC_WRAPPER, not meant to be called directly.");
        eprintln!();
        eprintln!("Usage: Add to .cargo/config.toml:");
        eprintln!("  [build]");
        eprintln!("  rustc-wrapper = \"masterror-rustc\"");
        exit(1);
    }

    // Cargo passes: $RUSTC_WRAPPER $RUSTC <args>
    // First argument is the path to rustc, rest are rustc arguments
    let rustc = &args[0];
    let rustc_args = &args[1..];

    let lang = detect_lang();
    let colored = supports_color();
    let registry = ErrorRegistry::new();

    let mut child = Command::new(rustc)
        .args(rustc_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("masterror-rustc: failed to run rustc: {e}");
            exit(1);
        });

    let stderr = child.stderr.take().expect("failed to capture stderr");
    let reader = BufReader::new(stderr);

    let mut stderr_handle = std::io::stderr().lock();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("masterror-rustc: read error: {e}");
                continue;
            }
        };

        // Output original line
        let _ = writeln!(stderr_handle, "{line}");

        // Check for error code pattern: error[E0308]
        if let Some(code) = extract_error_code(&line)
            && let Some(explanation) = get_explanation(registry, &code, lang, colored)
        {
            let _ = writeln!(stderr_handle, "{explanation}");
        }
    }

    drop(stderr_handle);

    let status = child.wait().unwrap_or_else(|e| {
        eprintln!("masterror-rustc: failed to wait for rustc: {e}");
        exit(1);
    });

    exit(status.code().unwrap_or(1));
}

/// Extract error code from line like "error[E0308]: mismatched types"
fn extract_error_code(line: &str) -> Option<String> {
    let start = line.find("error[E")?;
    let code_start = start + 6; // skip "error["
    let end = line[code_start..].find(']')?;
    Some(line[code_start..code_start + end].to_string())
}

/// Get translated explanation for error code
fn get_explanation(
    registry: &'static ErrorRegistry,
    code: &str,
    lang: Lang,
    colored: bool
) -> Option<String> {
    let entry = registry.find(code)?;
    let lang_code = lang.code();
    let mut output = String::new();

    // Header
    let title = entry.title.get(lang_code);
    if colored {
        output.push_str(&format!("\n   {} {}\n", "💡".bold(), title.bold().cyan()));
    } else {
        output.push_str(&format!("\n   💡 {title}\n"));
    }

    // Description
    let desc = entry.explanation.get(lang_code);
    if !desc.is_empty() {
        for line in desc.lines() {
            output.push_str(&format!("   {line}\n"));
        }
    }

    // Fix suggestions
    if !entry.fixes.is_empty() {
        output.push('\n');
        let fix_header = match lang.code() {
            "ru" => "Как исправить:",
            "ko" => "해결 방법:",
            _ => "How to fix:"
        };
        if colored {
            output.push_str(&format!("   {}\n", fix_header.bold().green()));
        } else {
            output.push_str(&format!("   {fix_header}\n"));
        }
        for fix in entry.fixes {
            let fix_desc = fix.description.get(lang_code);
            output.push_str(&format!("   • {fix_desc}\n"));
        }
    }

    Some(output)
}

/// Detect language from environment
fn detect_lang() -> Lang {
    if let Ok(lang) = env::var("MASTERROR_LANG") {
        return Lang::from_code(&lang);
    }

    if let Ok(lang) = env::var("LANG") {
        if lang.starts_with("ru") {
            return Lang::from_code("ru");
        }
        if lang.starts_with("ko") {
            return Lang::from_code("ko");
        }
    }

    Lang::En
}

/// Check if terminal supports colors
fn supports_color() -> bool {
    if env::var("NO_COLOR").is_ok() {
        return false;
    }
    if env::var("CLICOLOR_FORCE").is_ok() {
        return true;
    }
    is_stderr_tty()
}

/// Check if stderr is a TTY
#[cfg(unix)]
fn is_stderr_tty() -> bool {
    unsafe { libc::isatty(libc::STDERR_FILENO) != 0 }
}

#[cfg(not(unix))]
fn is_stderr_tty() -> bool {
    false
}
