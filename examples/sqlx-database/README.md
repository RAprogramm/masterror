<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# SQLx Database Error Handling Example

Demonstrates comprehensive database error handling patterns using SQLx and masterror.

## Features

- **Connection Error Handling** - Database connection failures
- **Query Error Mapping** - SQL errors to domain errors
- **Constraint Violation Handling** - Unique/foreign key violations
- **Transaction Error Patterns** - Rollback and commit handling
- **Row Not Found** - Handling missing data gracefully

## Running

```bash
cd examples/sqlx-database
cargo run
```

This example uses an in-memory SQLite database, so no external setup is required.

## Error Scenarios

### Connection Errors

```rust
// Mapped to AppError::Database
sqlx::SqlitePool::connect("sqlite::memory:").await?
```

### Row Not Found

```rust
// Mapped to AppError::NotFound
sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await?
```

### Unique Constraint Violation

```rust
// Mapped to AppError::Conflict
sqlx::query("INSERT INTO users (id, email) VALUES (?, ?)")
    .bind(id)
    .bind(email)
    .execute(&pool)
    .await?
```

### Transaction Handling

```rust
let mut tx = pool.begin().await?;

// Operations...

tx.commit().await?; // Or tx.rollback() on error
```

## Error Conversion

masterror automatically converts SQLx errors:

| SQLx Error | AppError Kind | HTTP Status |
|------------|---------------|-------------|
| `RowNotFound` | `NotFound` | 404 |
| `UniqueViolation` | `Conflict` | 409 |
| `ForeignKeyViolation` | `Conflict` | 409 |
| `ConnectionError` | `Database` | 500 |
| Other database errors | `Database` | 500 |

## Testing

```bash
cargo test
```

## License

MIT
