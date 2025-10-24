// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Basic async error handling with masterror and tokio.

use std::time::Duration;

use masterror::AppError;
use tokio::time::timeout;

/// Simulated data fetch operation
async fn fetch_data(id: u64) -> Result<String, AppError> {
    if id == 0 {
        return Err(AppError::validation("ID cannot be zero"));
    }

    if id > 1000 {
        return Err(AppError::not_found("ID not found in database"));
    }

    // Simulate async work
    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(format!("Data for ID {id}"))
}

/// Process fetched data
async fn process_data(data: &str) -> Result<String, AppError> {
    if data.is_empty() {
        return Err(AppError::validation("Data cannot be empty"));
    }

    // Simulate processing
    tokio::time::sleep(Duration::from_millis(50)).await;

    Ok(format!("Processed: {data}"))
}

/// Save processed result
async fn save_result(result: String) -> Result<(), AppError> {
    if result.len() > 1000 {
        return Err(AppError::bad_request("Result too large to save"));
    }

    // Simulate saving
    tokio::time::sleep(Duration::from_millis(50)).await;

    println!("✓ Saved: {result}");
    Ok(())
}

/// Complete processing pipeline
async fn process_pipeline(id: u64) -> Result<(), AppError> {
    println!("Processing ID {id}...");

    let data = fetch_data(id).await?;
    let processed = process_data(&data).await?;
    save_result(processed).await?;

    Ok(())
}

/// Slow operation for timeout demonstration
async fn slow_operation() -> Result<String, AppError> {
    tokio::time::sleep(Duration::from_secs(10)).await;
    Ok("Completed".to_string())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Basic Async Error Handling Example\\n");

    // Successful pipeline
    println!("=== Successful Pipeline ===");
    process_pipeline(123).await?;

    // Validation error
    println!("\\n=== Validation Error ===");
    match process_pipeline(0).await {
        Ok(()) => println!("✗ Should have failed"),
        Err(e) => {
            println!("✓ Expected error: {e}");
            println!("  → Kind: {:?}, HTTP: {}", e.kind, e.kind.http_status());
        }
    }

    // Not found error
    println!("\\n=== Not Found Error ===");
    match process_pipeline(9999).await {
        Ok(()) => println!("✗ Should have failed"),
        Err(e) => {
            println!("✓ Expected error: {e}");
            println!("  → Kind: {:?}, HTTP: {}", e.kind, e.kind.http_status());
        }
    }

    // Timeout error
    println!("\\n=== Timeout Error ===");
    match timeout(Duration::from_secs(1), slow_operation()).await {
        Ok(Ok(result)) => println!("✓ Completed: {result}"),
        Ok(Err(e)) => println!("✗ Operation error: {e}"),
        Err(e) => {
            let app_err: AppError = e.into();
            println!("✓ Expected timeout: {app_err}");
            println!(
                "  → Kind: {:?}, HTTP: {}",
                app_err.kind,
                app_err.kind.http_status()
            );
        }
    }

    println!("\\n✓ Example completed");
    Ok(())
}
