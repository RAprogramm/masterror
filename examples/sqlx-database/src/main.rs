// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! SQLx database error handling example.
//!
//! Demonstrates various database error scenarios and how masterror converts
//! SQLx errors into appropriate AppError types.

use masterror::AppError;
use sqlx::{Row, SqlitePool, sqlite::SqliteRow};

/// User model
#[derive(Debug, Clone)]
struct User {
    id:    i64,
    email: String,
    name:  String
}

/// Initialize database schema
async fn init_database(pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Insert a new user
async fn create_user(pool: &SqlitePool, email: &str, name: &str) -> Result<User, AppError> {
    let result = sqlx::query("INSERT INTO users (email, name) VALUES (?, ?)")
        .bind(email)
        .bind(name)
        .execute(pool)
        .await?;

    let id = result.last_insert_rowid();

    Ok(User {
        id,
        email: email.to_string(),
        name: name.to_string()
    })
}

/// Get user by ID
async fn get_user_by_id(pool: &SqlitePool, id: i64) -> Result<User, AppError> {
    let row: SqliteRow = sqlx::query("SELECT id, email, name FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(User {
        id:    row.get(0),
        email: row.get(1),
        name:  row.get(2)
    })
}

/// Get user by email
async fn get_user_by_email(pool: &SqlitePool, email: &str) -> Result<User, AppError> {
    let row: SqliteRow = sqlx::query("SELECT id, email, name FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(User {
        id:    row.get(0),
        email: row.get(1),
        name:  row.get(2)
    })
}

/// Update user name
async fn update_user(pool: &SqlitePool, id: i64, name: &str) -> Result<(), AppError> {
    let result = sqlx::query("UPDATE users SET name = ? WHERE id = ?")
        .bind(name)
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("user not found"));
    }

    Ok(())
}

/// Delete user
async fn delete_user(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("user not found"));
    }

    Ok(())
}

/// Transaction example: transfer operation
async fn transfer_user_data(
    pool: &SqlitePool,
    from_id: i64,
    to_email: &str
) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;

    // Get source user
    let row: SqliteRow = sqlx::query("SELECT name FROM users WHERE id = ?")
        .bind(from_id)
        .fetch_one(&mut *tx)
        .await?;

    let name: String = row.get(0);

    // Update destination user
    let result = sqlx::query("UPDATE users SET name = ? WHERE email = ?")
        .bind(&name)
        .bind(to_email)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("destination user not found"));
    }

    // Commit transaction
    tx.commit().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("SQLx Database Error Handling Example\\n");

    // Connect to in-memory SQLite database
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    println!("✓ Connected to database");

    // Initialize schema
    init_database(&pool).await?;
    println!("✓ Database schema initialized\\n");

    // Create users
    println!("=== Creating Users ===");
    let user1 = create_user(&pool, "alice@example.com", "Alice").await?;
    println!("✓ Created user: {} (ID: {})", user1.name, user1.id);

    let user2 = create_user(&pool, "bob@example.com", "Bob").await?;
    println!("✓ Created user: {} (ID: {})", user2.name, user2.id);

    // Try to create duplicate email (Conflict error)
    println!("\\n=== Testing Unique Constraint Violation ===");
    match create_user(&pool, "alice@example.com", "Alice Duplicate").await {
        Ok(_) => println!("✗ Should have failed with conflict"),
        Err(e) => {
            println!("✓ Expected error: {}", e);
            println!("  → Kind: {:?}, HTTP: {}", e.kind, e.kind.http_status());
        }
    }

    // Get existing user
    println!("\\n=== Retrieving User ===");
    let found = get_user_by_email(&pool, "alice@example.com").await?;
    println!("✓ Found user: {} ({})", found.name, found.email);

    // Try to get non-existent user (NotFound error)
    println!("\\n=== Testing Row Not Found ===");
    match get_user_by_id(&pool, 999).await {
        Ok(_) => println!("✗ Should have failed with not found"),
        Err(e) => {
            println!("✓ Expected error: {}", e);
            println!("  → Kind: {:?}, HTTP: {}", e.kind, e.kind.http_status());
        }
    }

    // Update user
    println!("\\n=== Updating User ===");
    update_user(&pool, user1.id, "Alice Updated").await?;
    let updated = get_user_by_id(&pool, user1.id).await?;
    println!("✓ Updated user name: {}", updated.name);

    // Try to update non-existent user
    println!("\\n=== Testing Update on Non-existent User ===");
    match update_user(&pool, 999, "Ghost").await {
        Ok(_) => println!("✗ Should have failed with not found"),
        Err(e) => {
            println!("✓ Expected error: {}", e);
            println!("  → Kind: {:?}, HTTP: {}", e.kind, e.kind.http_status());
        }
    }

    // Transaction example
    println!("\\n=== Testing Transaction ===");
    transfer_user_data(&pool, user1.id, "bob@example.com").await?;
    let bob_updated = get_user_by_email(&pool, "bob@example.com").await?;
    println!(
        "✓ Transaction completed: Bob's name is now '{}'",
        bob_updated.name
    );

    // Delete user
    println!("\\n=== Deleting User ===");
    delete_user(&pool, user2.id).await?;
    println!("✓ Deleted user with ID: {}", user2.id);

    // Try to delete again (NotFound)
    match delete_user(&pool, user2.id).await {
        Ok(_) => println!("✗ Should have failed with not found"),
        Err(e) => {
            println!("✓ Expected error: {}", e);
            println!("  → Kind: {:?}, HTTP: {}", e.kind, e.kind.http_status());
        }
    }

    pool.close().await;
    println!("\\n✓ Database connection closed");

    Ok(())
}
