// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Migration from thiserror to masterror - incremental steps.

use masterror::{AppError, AppResult, field};

fn main() {
    println!("Migration from thiserror to masterror");
    println!();
    println!("Step 1: Change `thiserror::Error` -> `masterror::Error`");
    println!("Step 2: Add #[app_error(...)] for HTTP/gRPC mapping");
    println!("Step 3: Use AppError with structured metadata");
    println!();
    match find_user("alice") {
        Ok(()) => println!("User found"),
        Err(e) => println!("Error: {e}")
    }
}

fn find_user(user_id: &str) -> AppResult<()> {
    Err(AppError::not_found(format!("User {user_id} not found"))
        .with_field(field::str("user_id", user_id.to_string())))
}
