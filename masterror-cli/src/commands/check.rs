// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Check command - run cargo check and explain errors.

use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio}
};

use crate::{locale::Locale, options::DisplayOptions, output, parser::CargoMessage};

/// Run cargo check and explain errors.
pub fn run(
    locale: &Locale,
    args: &[String],
    opts: &DisplayOptions
) -> Result<(), Box<dyn std::error::Error>> {
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

    let stdout = cmd.stdout.take().expect("stdout captured");
    let reader = BufReader::new(stdout);

    let mut error_count = 0;

    for line in reader.lines() {
        let line = line?;
        if let Ok(msg) = serde_json::from_str::<CargoMessage>(&line)
            && msg.is_error()
        {
            error_count += 1;
            output::print_error(locale.lang(), &msg, opts);
            println!();
        }
    }

    let status = cmd.wait()?;

    if error_count > 0 {
        println!("Found {error_count} error(s). Run `masterror explain <code>` for details.");
    }

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
