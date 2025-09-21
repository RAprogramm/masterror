# Rust error handling basics

This page explains the building blocks of error handling in Rust. The goal is
to make the rest of the wiki easier to follow, even if you are new to the
language.

## Terminology

- **`Result<T, E>`** is an enum with two variants: `Ok(T)` holds a successful
  value, and `Err(E)` holds an error. Every function that can fail should return
  a `Result`.
- **`?` operator** unwraps an `Ok` value or returns early with the `Err` variant.
  It works with any `Result` or `Option` and is the primary way to propagate
  errors.
- **`std::error::Error` trait** describes types that behave like errors. Most
  libraries implement it for their failure types. Implementing `Error` allows
  your type to integrate with logging, conversions, and `anyhow`.
- **`From<E>`/`Into<E>` conversions** are how one error turns into another.
  When the `?` operator sees an `Err`, it uses `From` to convert between error
  types automatically.

## Writing a fallible function

The following example downloads JSON from an in-memory HTTP server and parses a
field. It uses standard library errors and propagates them with `?`.

```rust
use std::collections::HashMap;

fn read_flag(data: &str) -> Result<bool, serde_json::Error> {
    let payload: HashMap<String, serde_json::Value> = serde_json::from_str(data)?;
    let flag = payload
        .get("feature_enabled")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    Ok(flag)
}

fn parse_response(response: &str) -> Result<bool, ReadFlagError> {
    let enabled = read_flag(response)?;
    Ok(enabled)
}

#[derive(Debug, thiserror::Error)]
#[error("failed to parse feature flag: {source}")]
pub struct ReadFlagError {
    #[from]
    source: serde_json::Error,
}

fn main() -> Result<(), ReadFlagError> {
    let json = r#"{ "feature_enabled": true }"#;
    let flag = parse_response(json)?;
    assert!(flag);
    Ok(())
}
```

Key observations:

1. `read_flag` returns `Result<bool, serde_json::Error>` because JSON parsing can
   fail. Nothing special is required — the compiler enforces handling the error.
2. `parse_response` returns a custom `ReadFlagError` that wraps the parsing
   error. The `?` operator converts the JSON error into `ReadFlagError` via the
   `#[from]` attribute.
3. `main` uses `Result` as its return type. If an error occurs, the program exits
   with a non-zero status code and prints the error.

## Recovering from errors

Not every error should bubble up. Use `match`, `if let`, or helper methods to
inspect and recover when possible.

```rust
fn recover_or_default(data: &str) -> bool {
    match read_flag(data) {
        Ok(flag) => flag,
        Err(err) => {
            tracing::warn!(error = %err, "invalid feature flag payload");
            false
        }
    }
}
```

Rust encourages explicit recovery paths, so code remains predictable even when a
failure happens.

## Mapping one error into another

Applications frequently hide implementation details behind domain-specific
errors. `map_err` is a lightweight way to translate errors without introducing
new types.

```rust
fn read_flag_for_user(data: &str, user_id: u64) -> Result<bool, AppError> {
    read_flag(data).map_err(|err| {
        masterror::AppError::bad_request(
            format!("user {user_id} sent invalid JSON: {err}"),
        )
    })
}
```

`map_err` receives the original error and lets you convert it into an
application-level type — here we produce an HTTP 400 error using `masterror`'s
helper. The next pages expand on this technique and show how to avoid allocating
new `String`s by using structured conversions.
