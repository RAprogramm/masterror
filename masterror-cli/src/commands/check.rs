// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Check command - run cargo check and explain errors.

use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio}
};

use masterror_knowledge::Lang;

use crate::{
    error::{AppError, Result},
    options::DisplayOptions,
    output,
    parser::CargoMessage
};

/// Allowed cargo arguments whitelist.
const ALLOWED_ARGS: &[&str] = &[
    "--release",
    "--all-targets",
    "--all-features",
    "--no-default-features",
    "-p",
    "--package",
    "--workspace",
    "--lib",
    "--bins",
    "--bin",
    "--examples",
    "--example",
    "--tests",
    "--test",
    "--benches",
    "--bench",
    "--features",
    "-F",
    "--target",
    "--profile",
    "-j",
    "--jobs",
    "-v",
    "--verbose",
    "-q",
    "--quiet",
    "--locked",
    "--frozen",
    "--offline"
];

/// Validate cargo arguments against whitelist.
fn validate_args(args: &[String]) -> Result<()> {
    for arg in args {
        if arg.starts_with('-') {
            let is_allowed = ALLOWED_ARGS
                .iter()
                .any(|allowed| arg == *allowed || arg.starts_with(&format!("{allowed}=")));
            if !is_allowed {
                return Err(AppError::InvalidArgument {
                    arg: arg.clone()
                });
            }
        }
    }
    Ok(())
}

/// Run cargo check and explain errors.
pub fn run(lang: Lang, args: &[String], opts: &DisplayOptions) -> Result<()> {
    validate_args(args)?;

    let msg_format = if opts.colored {
        "--message-format=json-diagnostic-rendered-ansi"
    } else {
        "--message-format=json"
    };

    let mut cmd = Command::new("cargo")
        .arg("check")
        .arg(msg_format)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdout = cmd
        .stdout
        .take()
        .ok_or_else(|| AppError::Io(std::io::Error::other("failed to capture stdout")))?;
    let reader = BufReader::new(stdout);

    let mut error_count = 0;

    for line in reader.lines() {
        let line = line?;
        if let Ok(msg) = serde_json::from_str::<CargoMessage>(&line)
            && msg.is_error()
        {
            error_count += 1;
            output::print_error(lang, &msg, opts);
            println!();
        }
    }

    let status = cmd.wait()?;

    if error_count > 0 {
        println!("Found {error_count} error(s). Run `masterror explain <code>` for details.");
    }

    if !status.success() {
        match status.code() {
            Some(code) => {
                return Err(AppError::CargoFailed {
                    code
                });
            }
            None => return Err(AppError::CargoSignaled)
        }
    }

    Ok(())
}
