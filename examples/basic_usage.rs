// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Basic usage example showing the simplest ways to create and use errors.
//!
//! Run with:
//! ```sh
//! cargo run --example basic_usage
//! ```

use masterror::{AppError, AppErrorKind, AppResult, field};

fn validate_age(age: i32) -> AppResult<()> {
    masterror::ensure!(age >= 0, AppError::bad_request("Age cannot be negative"));
    masterror::ensure!(age <= 150, AppError::bad_request("Age seems unrealistic"));
    Ok(())
}

fn fetch_user(id: &str) -> AppResult<String> {
    if id.is_empty() {
        masterror::fail!(AppError::bad_request("User ID cannot be empty"));
    }

    if id == "404" {
        return Err(AppError::not_found("User not found"));
    }

    Ok(format!("User {id}"))
}

fn process_request(user_id: &str, age: i32) -> AppResult<String> {
    validate_age(age)?;
    let user = fetch_user(user_id)?;
    Ok(format!("{user} is {age} years old"))
}

fn database_operation() -> AppResult<()> {
    let err = AppError::database_with_message("Connection failed")
        .with_field(field::str("host", "localhost"))
        .with_field(field::u64("port", 5432))
        .with_field(field::duration(
            "timeout",
            std::time::Duration::from_secs(5)
        ));

    Err(err)
}

fn main() {
    println!("=== Basic Error Creation ===\n");

    let err = AppError::new(AppErrorKind::BadRequest, "Invalid input");
    println!("Simple error: {err}");
    println!("Error kind: {:?}", err.kind);
    println!("Error code: {:?}\n", err.code);

    println!("=== Using ensure! macro ===\n");

    match validate_age(-5) {
        Ok(()) => println!("Age valid"),
        Err(e) => println!("Validation failed: {e}")
    }

    match validate_age(200) {
        Ok(()) => println!("Age valid"),
        Err(e) => println!("Validation failed: {e}")
    }

    match validate_age(25) {
        Ok(()) => println!("Age 25 is valid\n"),
        Err(e) => println!("Validation failed: {e}")
    }

    println!("=== Using fail! macro ===\n");

    match fetch_user("") {
        Ok(user) => println!("Found: {user}"),
        Err(e) => println!("Fetch failed: {e}")
    }

    match fetch_user("404") {
        Ok(user) => println!("Found: {user}"),
        Err(e) => println!("Fetch failed: {e}")
    }

    match fetch_user("alice") {
        Ok(user) => println!("Found: {user}\n"),
        Err(e) => println!("Fetch failed: {e}")
    }

    println!("=== Error Propagation ===\n");

    match process_request("bob", 30) {
        Ok(result) => println!("Success: {result}"),
        Err(e) => println!("Request failed: {e}")
    }

    match process_request("404", 30) {
        Ok(result) => println!("Success: {result}"),
        Err(e) => println!("Request failed: {e}\n")
    }

    println!("=== Structured Metadata ===\n");

    match database_operation() {
        Ok(()) => println!("Database operation succeeded"),
        Err(e) => {
            println!("Database error: {e}");
            println!("Metadata fields: {}", e.metadata().len());
            for (key, value) in e.metadata().iter() {
                println!("  {key}: {value:?}");
            }
        }
    }
}
