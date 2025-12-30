// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Demonstrates colored terminal output for errors.
//!
//! This example shows how the `colored` feature enhances error visibility
//! in CLI applications with automatic TTY detection and professional color
//! coding.
//!
//! # Running
//!
//! ```bash
//! # With colors (when stdout is a TTY)
//! cargo run --example colored_cli --features colored
//!
//! # Without colors (when piped)
//! cargo run --example colored_cli --features colored | cat
//!
//! # Disable colors via environment
//! NO_COLOR=1 cargo run --example colored_cli --features colored
//! ```

use std::io::Error as IoError;

use masterror::{AppError, AppErrorKind, field};

fn main() {
    println!("=== Masterror Colored CLI Demo ===\n");
    demo_critical_errors();
    demo_client_errors();
    demo_error_with_context();
    demo_error_with_metadata();
    demo_error_chain();
    demo_all_error_kinds();
    println!("\n=== Demo Complete ===");
    println!("\nColor behavior:");
    println!("  - Critical errors (5xx): Red");
    println!("  - Client errors (4xx): Yellow");
    println!("  - Error codes: Cyan");
    println!("  - Messages: Bright white");
    println!("  - Source context: Dimmed");
    println!("  - Metadata keys: Green");
}

fn demo_critical_errors() {
    println!("--- Critical Server Errors (5xx) ---\n");
    let errors = vec![
        AppError::internal("Database connection pool exhausted"),
        AppError::database_with_message("Failed to execute migration"),
        AppError::timeout("API request exceeded 30s timeout"),
        AppError::network("DNS resolution failed for api.example.com"),
    ];
    for err in errors {
        eprintln!("{}\n", err);
    }
}

fn demo_client_errors() {
    println!("--- Client Errors (4xx) ---\n");
    let errors = vec![
        AppError::not_found("User with ID 12345 does not exist"),
        AppError::bad_request("Missing required field: email"),
        AppError::validation("Password must be at least 8 characters"),
        AppError::forbidden("Insufficient permissions to access resource"),
    ];
    for err in errors {
        eprintln!("{}\n", err);
    }
}

fn demo_error_with_context() {
    println!("--- Error with Source Context ---\n");
    let io_err = IoError::other("Connection reset by peer");
    let err = AppError::network("Failed to fetch user data").with_context(io_err);
    eprintln!("{}\n", err);
}

fn demo_error_with_metadata() {
    println!("--- Error with Structured Metadata ---\n");
    let err = AppError::database_with_message("Query execution failed")
        .with_field(field::str("query", "SELECT * FROM users WHERE id = ?"))
        .with_field(field::u64("duration_ms", 5432))
        .with_field(field::str("connection_id", "conn_abc123"))
        .with_field(field::u64("retry_count", 3));
    eprintln!("{}\n", err);
}

fn demo_error_chain() {
    println!("--- Deep Error Chain ---\n");
    let root = IoError::other("Disk full");
    let mid = format!("Failed to write log file: {}", root);
    let top = AppError::internal("Application initialization failed")
        .with_context(IoError::other(mid))
        .with_field(field::str("config_path", "/etc/app/config.toml"))
        .with_field(field::u64("retry_attempt", 3));
    eprintln!("{}\n", top);
}

fn demo_all_error_kinds() {
    println!("--- All Error Kinds (Sampling) ---\n");
    let kinds = vec![
        (AppErrorKind::NotFound, "Resource not found"),
        (AppErrorKind::Validation, "Input validation failed"),
        (AppErrorKind::Conflict, "Resource already exists"),
        (AppErrorKind::Unauthorized, "Authentication required"),
        (AppErrorKind::Forbidden, "Access denied"),
        (AppErrorKind::Internal, "Internal server error"),
        (AppErrorKind::Database, "Database operation failed"),
        (AppErrorKind::Timeout, "Operation timed out"),
        (AppErrorKind::Network, "Network error"),
        (AppErrorKind::RateLimited, "Too many requests"),
    ];
    for (kind, msg) in kinds {
        let err = AppError::new(kind, msg);
        eprintln!("{}\n", err);
    }
}
