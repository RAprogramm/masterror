// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Structured metadata example showing typed fields vs string formatting.
//!
//! Run with:
//! ```sh
//! cargo run --example structured_metadata
//! ```

use std::{net::IpAddr, time::Duration};

use masterror::{AppError, AppResult, field};

fn database_query(
    table: &'static str,
    user_id: u64,
    timeout: Duration,
    retry_count: u64
) -> AppResult<String> {
    Err(AppError::database_with_message("Query failed")
        .with_field(field::str("table", table))
        .with_field(field::u64("user_id", user_id))
        .with_field(field::duration("timeout", timeout))
        .with_field(field::u64("retry_count", retry_count)))
}

fn api_request(endpoint: &'static str, client_ip: IpAddr, latency_ms: f64) -> AppResult<()> {
    Err(AppError::external_api("External API call failed")
        .with_field(field::str("endpoint", endpoint))
        .with_field(field::ip("client_ip", client_ip))
        .with_field(field::f64("latency_ms", latency_ms)))
}

fn main() {
    println!("=== Structured Metadata with Typed Fields ===\n");

    match database_query("users", 12345, Duration::from_secs(30), 3) {
        Ok(_) => println!("Query succeeded"),
        Err(e) => {
            println!("Error: {e}");
            println!("\nMetadata:");
            for (key, value) in e.metadata().iter() {
                println!("  {key}: {value:?}");
            }
        }
    }

    println!("\n=== API Request with IP and Float ===\n");

    let client_ip: IpAddr = "192.168.1.100".parse().unwrap();
    match api_request("/api/users", client_ip, 123.45) {
        Ok(()) => println!("API request succeeded"),
        Err(e) => {
            println!("Error: {e}");
            println!("\nMetadata fields: {}", e.metadata().len());
            for (key, value) in e.metadata().iter() {
                println!("  {key}: {value:?}");
            }
        }
    }

    println!("\n=== Multiple Chained Metadata ===\n");

    let err = AppError::internal("Processing failed")
        .with_field(field::str("stage", "validation"))
        .with_field(field::u64("record_id", 999))
        .with_field(field::duration("elapsed", Duration::from_millis(456)))
        .with_field(field::bool("retryable", true));

    println!("Error: {err}");
    println!("Total metadata fields: {}", err.metadata().len());
    println!("\nAll fields:");
    for (key, value) in err.metadata().iter() {
        println!("  {key} = {value:?}");
    }
}
