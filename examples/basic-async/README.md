<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Basic Async Example

Simple demonstration of using masterror in async Rust code with tokio.

## Features

- **Async Error Handling** - Using `?` operator in async functions
- **Timeout Handling** - Converting tokio timeout errors to AppError
- **Error Propagation** - Clean error propagation through async call chains
- **Result Types** - Using `AppResult<T>` for async operations

## Running

```bash
cd examples/basic-async
cargo run
```

## Key Concepts

### Async Functions with AppError

```rust
async fn fetch_data(id: u64) -> Result<String, AppError> {
    if id == 0 {
        return Err(AppError::validation("ID cannot be zero"));
    }

    Ok(format!("Data for ID {}", id))
}
```

### Timeout Error Handling

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(5),
    fetch_data(123)
).await?; // Converts Elapsed to AppError::Timeout
```

### Error Propagation

```rust
async fn process() -> Result<(), AppError> {
    let data = fetch_data(123).await?;
    let result = process_data(&data).await?;
    save_result(result).await?;
    Ok(())
}
```

## License

MIT
