# When to use `thiserror`, `anyhow`, or `masterror`

Rust gives you multiple complementary error crates. This page compares how they
behave and shows concrete examples.

## Quick summary

| Crate        | Primary goal                                    | Typical usage stage             |
|--------------|--------------------------------------------------|---------------------------------|
| `thiserror`  | Define strongly typed domain errors with `derive`| Library and boundary layers     |
| `anyhow`     | Prototype quickly with dynamic context           | CLI tools, experiments, glue    |
| `masterror`  | Ship stable API responses with rich metadata     | Web backends, public interfaces |

You can mix them: `masterror` re-exports the derive macro so you keep using
`#[derive(Error)]`, and you can attach `anyhow::Error` as context on
`AppError`.

## Example: modelling a domain error

The following snippet derives a typed error using each crate.

```rust
use masterror::{AppCode, AppError, AppErrorKind, Error};
use serde::Deserialize;
use thiserror::Error as ThisError;

#[derive(Debug, Deserialize)]
struct Payload {
    flag: bool,
}

#[derive(Debug, Error)]
#[error("invalid payload: {source}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::new("INVALID_PAYLOAD"))]
struct PayloadError {
    #[from]
    #[source]
    source: serde_json::Error,
}

fn parse_with_masterror(input: &str) -> masterror::AppResult<Payload> {
    let payload: Payload = serde_json::from_str(input).map_err(PayloadError::from)?;
    Ok(payload)
}

#[derive(Debug, ThisError)]
#[error("invalid payload: {source}")]
struct PlainPayloadError {
    #[from]
    source: serde_json::Error,
}

fn parse_with_thiserror(input: &str) -> Result<Payload, PlainPayloadError> {
    let payload = serde_json::from_str(input)?;
    Ok(payload)
}

fn parse_with_anyhow(input: &str) -> Result<Payload, anyhow::Error> {
    let payload = serde_json::from_str::<Payload>(input)
        .map_err(|err| anyhow::anyhow!("invalid payload: {err}"))?;
    Ok(payload)
}

fn convert_anyhow_into_masterror(err: anyhow::Error) -> AppError {
    AppError::internal("unexpected parser failure").with_context(err)
}
```

Observations:

- `thiserror` focuses on ergonomic derives and string formatting, but it does
  not impose how callers expose the error to clients.
- `anyhow` stores a dynamic error with a backtrace. It is ideal for prototyping
  and small CLIs, but it does not convey HTTP status codes or machine-readable
  metadata.
- `masterror` is opinionated about API boundaries. By using `#[app_error]`, the
  domain error maps to a stable `AppErrorKind` and `AppCode` automatically.

## Mapping errors at service boundaries

Imagine an Axum handler that validates JSON, queries a database, and reaches an
external API. Each crate offers different trade-offs.

```rust
async fn handler_with_anyhow() -> Result<String, anyhow::Error> {
    let payload = parse_with_anyhow("{ }")?;
    Ok(format!("flag: {}", payload.flag))
}

async fn handler_with_thiserror() -> Result<String, PlainPayloadError> {
    let payload = parse_with_thiserror("{ }")?;
    Ok(format!("flag: {}", payload.flag))
}

async fn handler_with_masterror() -> masterror::AppResult<String> {
    let payload = parse_with_masterror("{ }")?;
    Ok(format!("flag: {}", payload.flag))
}
```

- The `anyhow` version surfaces a stringified error and a backtrace. Clients
  receive HTTP 500 unless you write custom mapping logic.
- The `thiserror` version returns a typed error, but you still have to convert it
  into an HTTP response yourself.
- The `masterror` version already contains the HTTP 400 classification, a stable
  `AppCode`, and optional JSON details.

## Attaching context across crates

You can combine the strengths of each crate. Keep `thiserror` derives for rich
messages, wrap the result in `AppError`, and use `anyhow` for debug traces when
needed.

```rust
fn load_configuration(path: &std::path::Path) -> masterror::AppResult<String> {
    let contents = std::fs::read_to_string(path).map_err(|err| {
        AppError::internal("failed to read configuration")
            .with_code(AppCode::new("CONFIG_IO"))
            .with_context(anyhow::Error::from(err))
    })?;
    Ok(contents)
}
```

If the configuration source must encode per-environment values in the code, use
`AppCode::try_new` to build the identifier dynamically and bubble up
`ParseAppCodeError` when validation fails.

`AppError` stores the `anyhow::Error` internally without exposing it to clients.
`with_context` reuses any shared `Arc` handles provided by upstream crates, so
you preserve pointer identity without extra allocations. You still emit clean
JSON responses, while logs retain the full diagnostic payload.

## Why choose `masterror`

1. **Stable contract.** `AppErrorKind` and `ErrorResponse` stay consistent across
   services, making cross-team collaboration easier.
2. **Framework adapters.** Ready-to-use integrations with Axum, Actix Web,
   `utoipa`, `serde_json`, and others remove boilerplate.
3. **Structured metadata.** Attach retry hints, authentication challenges, and
   JSON details without building ad-hoc enums.
4. **Derive support.** Reuse the familiar `thiserror` syntax via
   `masterror::Error` and augment it with `#[app_error]` rules.
5. **Context preservation.** Store source errors (including `anyhow::Error`) for
   logging, while presenting sanitized messages externally.

Use `anyhow` when speed matters more than structure, `thiserror` when crafting
libraries, and `masterror` when you need predictable, well-documented API
responses.
