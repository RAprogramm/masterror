// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Migration from anyhow to masterror - same ergonomics, better features.

use std::fs;

use masterror::{AppError, AppResult, ResultExt, ensure, fail, field};

fn main() {
    println!("Migration from anyhow to masterror");
    println!();
    println!("Replace: anyhow::Result -> masterror::AppResult");
    println!("Replace: bail!() -> fail!()");
    println!("Same:    ensure!() works identically");
    println!("Same:    .context() works identically");
    println!("Plus:    Structured metadata with field::*");
    println!();

    match read_config("/tmp/config.toml") {
        Ok(content) => println!("Config: {content}"),
        Err(e) => println!("Error: {e}")
    }
}

fn read_config(path: &str) -> AppResult<String> {
    // .context() works exactly like anyhow
    let content = fs::read_to_string(path).context("Failed to read config file")?;

    ensure!(
        !content.is_empty(),
        AppError::bad_request("Config file is empty")
            .with_field(field::str("path", path.to_string()))
    );

    if content.starts_with("invalid") {
        fail!(
            AppError::bad_request("Invalid config format")
                .with_field(field::str("path", path.to_string()))
                .with_field(field::u64("size", content.len() as u64))
        );
    }

    Ok(content)
}
