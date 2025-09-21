# Building applications with `masterror`

`masterror` provides a stable taxonomy for API-driven services. This page shows
how to define domain errors, propagate them through business logic, and surface
them as structured responses.

## Core types at a glance

- `AppErrorKind` categorises a failure (`BadRequest`, `Unauthorized`,
  `Validation`, `Internal`, ...). Each kind maps to a conservative HTTP status.
- `AppCode` is an optional machine-readable identifier for your API clients.
- `AppError` bundles a kind, developer message, optional `AppCode`, optional
  structured details, and retry/authentication hints.
- `AppResult<T>` is a convenient alias for `Result<T, AppError>`.

Use the helpers to construct errors without allocating intermediate `String`s.

```rust
use masterror::{AppError, AppErrorKind, AppResult};

pub fn ensure_flag(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::bad_request("flag must be enabled"));
    }
    Ok(())
}

pub fn get_secret(flag: bool) -> AppResult<&'static str> {
    ensure_flag(flag)?;
    Ok("swordfish")
}
```

`AppError::bad_request` returns an HTTP 400 response. Other helpers include
`AppError::internal`, `AppError::timeout`, `AppError::unauthorized`, and more.

## Attaching codes and structured details

Attach machine-friendly metadata so clients can branch on errors without parsing
text.

```rust
use masterror::{AppCode, AppError};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct MissingField<'a> {
    field: &'a str,
}

pub fn parse_payload(json: &str) -> masterror::AppResult<&str> {
    let payload: serde_json::Value = serde_json::from_str(json).map_err(|err| {
        AppError::bad_request("payload must be valid JSON")
            .with_code(AppCode::new("INVALID_JSON"))
            .with_details(&MissingField { field: "feature_flag" })
            .with_context(err)
    })?;

    payload
        .get("feature_flag")
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            AppError::bad_request("feature_flag string is required")
                .with_code(AppCode::new("MISSING_FIELD"))
        })
}
```

`with_context` stores the original `serde_json::Error` for logging; clients only
see the sanitized message, code, and JSON details.

## Deriving domain errors

Combine `masterror::Error` derive macros with `#[app_error]` to convert domain
errors into `AppError` automatically.

```rust
use masterror::{AppCode, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("database query failed: {source}")]
#[app_error(kind = AppErrorKind::Database, code = AppCode::new("DB_FAILURE"))]
pub struct DatabaseFailure {
    #[from]
    #[source]
    source: sqlx_core::Error,
}

pub async fn load_user(pool: &sqlx_core::pool::PoolConnection<'_, sqlx_core::Postgres>)
    -> Result<(), DatabaseFailure>
{
    Err(sqlx_core::Error::RowNotFound)?;
    Ok(())
}
```

Whenever `DatabaseFailure` is converted into `AppError`, the derived impl picks
`AppErrorKind::Database` and attaches the `DB_FAILURE` code. No manual mapping is
required in handlers.

## Framework integrations

Enable the relevant feature to integrate with HTTP frameworks:

- `axum`: `AppError` implements `IntoResponse` to emit JSON bodies that follow
  `ErrorResponse` (status, code, message, optional details/retry info).
- `actix`: `AppError` implements `ResponseError` with the same JSON schema.
- `openapi`: `ErrorResponse` gains `utoipa::ToSchema` so your OpenAPI spec stays
  in sync.

Example Axum handler:

```rust
use axum::{routing::get, Router};
use masterror::AppError;

async fn handler() -> masterror::AppResult<&'static str> {
    Err(AppError::unauthorized("missing token"))
}

fn app() -> Router {
    Router::new().route("/", get(handler))
}
```

Axum automatically converts the error into an HTTP 401 JSON payload.

## Logging and telemetry

`AppError` implements `std::error::Error`. Use `tracing` to log errors once, at
module boundaries (e.g., HTTP middleware or background task entry points).

```rust
fn log_error(err: &masterror::AppError) {
    tracing::error!(kind = ?err.kind, code = ?err.code, "request failed");
    if let Some(context) = err.context() {
        tracing::debug!(?context, "captured error context");
    }
}
```

Avoid logging the same error multiple times — the structured data already
contains everything needed for observability dashboards.

## Testing error behaviour

Write unit tests that assert on the `AppErrorKind`, optional `AppCode`, and the
serialised `ErrorResponse` payload.

```rust
#[test]
fn missing_field_is_bad_request() {
    let err = parse_payload("{}").unwrap_err();
    assert!(matches!(err.kind, AppErrorKind::BadRequest));
    assert_eq!(err.code.unwrap().as_str(), "MISSING_FIELD");

    let response: masterror::ErrorResponse = err.clone().into();
    assert_eq!(response.status.as_u16(), 400);
}
```

Cloning is cheap because `AppError` stores data on the stack and shares context
via `Arc` under the hood. Use these assertions to guarantee stable APIs.
