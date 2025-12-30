// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Derive Error example showing thiserror compatibility and AppError mapping.
//!
//! Run with:
//! ```sh
//! cargo run --example derive_error
//! ```

use std::{error::Error as StdError, io};

use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("I/O failed: {source}")]
struct IoWrapperError {
    #[from]
    #[source]
    source: io::Error
}

#[derive(Debug, Error)]
#[error("User {user_id} not found")]
#[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound, message)]
struct UserNotFound {
    user_id: String
}

#[derive(Debug, Error)]
#[error("Database connection failed")]
#[app_error(kind = AppErrorKind::Database, code = AppCode::Database)]
struct DatabaseError {
    #[source]
    cause: io::Error
}

#[derive(Debug, Error)]
enum ServiceError {
    #[error("Authentication failed: {reason}")]
    #[app_error(kind = AppErrorKind::Unauthorized, code = AppCode::Unauthorized, message)]
    AuthFailed { reason: String },

    #[error("Rate limit exceeded")]
    #[app_error(kind = AppErrorKind::RateLimited, code = AppCode::RateLimited, message)]
    RateLimited,

    #[error(transparent)]
    #[app_error(kind = AppErrorKind::Database, code = AppCode::Database)]
    Database(#[from] DatabaseError)
}

fn simulate_io_error() -> Result<(), IoWrapperError> {
    Err(io::Error::other("disk offline").into())
}

fn find_user(user_id: &str) -> Result<String, UserNotFound> {
    if user_id.is_empty() {
        return Err(UserNotFound {
            user_id: user_id.to_string()
        });
    }
    Ok(format!("User: {user_id}"))
}

fn connect_database() -> Result<(), DatabaseError> {
    Err(DatabaseError {
        cause: io::Error::other("connection refused")
    })
}

fn authenticate(valid: bool) -> Result<(), ServiceError> {
    if !valid {
        return Err(ServiceError::AuthFailed {
            reason: "invalid token".to_string()
        });
    }
    Ok(())
}

fn main() {
    println!("=== thiserror Compatibility ===\n");
    match simulate_io_error() {
        Ok(()) => println!("I/O succeeded"),
        Err(e) => {
            println!("Error: {e}");
            println!("Source: {:?}", e.source());
        }
    }
    println!("\n=== AppError Mapping ===\n");
    match find_user("") {
        Ok(user) => println!("Found: {user}"),
        Err(e) => {
            println!("Domain error: {e}");
            let app_error: AppError = e.into();
            println!("AppError kind: {:?}", app_error.kind);
            println!("AppError code: {:?}", app_error.code);
            println!("Message exposed: {:?}", app_error.message);
        }
    }
    println!("\n=== Error Source Chain ===\n");
    match connect_database() {
        Ok(()) => println!("Connected"),
        Err(e) => {
            println!("Domain error: {e}");
            let app_error: AppError = e.into();
            println!("Has source: {}", app_error.source_ref().is_some());
            if let Some(source) = app_error.source_ref() {
                println!("Source: {source}");
            }
        }
    }
    println!("\n=== Enum Variants ===\n");
    match authenticate(false) {
        Ok(()) => println!("Authenticated"),
        Err(e) => {
            println!("Service error: {e}");
            let app_error: AppError = e.into();
            println!("Kind: {:?}", app_error.kind);
            println!("Code: {:?}", app_error.code);
        }
    }
    let rate_limit_err = ServiceError::RateLimited;
    println!("\nRate limit error: {rate_limit_err}");
    let app_error: AppError = rate_limit_err.into();
    println!("Kind: {:?}", app_error.kind);
    match connect_database().map_err(ServiceError::from) {
        Ok(()) => println!("Connected"),
        Err(e) => {
            println!("\nService error: {e}");
            let app_error: AppError = e.into();
            println!("Kind: {:?}", app_error.kind);
        }
    }
}
