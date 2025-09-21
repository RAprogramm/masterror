# Patterns and troubleshooting

This page collects recipes for common error-handling tasks along with debugging
strategies.

## Mapping third-party errors

Prefer typed conversions over string formatting. `masterror` exposes helper
constructors and feature-gated conversions.

```rust
use masterror::{AppCode, AppError};

pub async fn fetch_user(client: &reqwest::Client) -> masterror::AppResult<String> {
    let response = client.get("https://example.com/user").send().await.map_err(|err| {
        AppError::external_api("failed to reach user service")
            .with_code(AppCode::new("UPSTREAM_HTTP"))
            .with_context(err)
    })?;

    response.text().await.map_err(|err| {
        AppError::external_api("failed to decode response body").with_context(err)
    })
}
```

Enable the `reqwest` feature to classify timeouts and HTTP status codes
automatically. Similar conversions exist for `sqlx`, `redis`, `validator`,
`config`, and more.

## Validating inputs

Surface validation failures as structured data so clients can highlight fields.

```rust
use masterror::{AppCode, AppError};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct CreateUser {
    #[validate(length(min = 3))]
    username: String,

    #[validate(email)]
    email: String,
}

pub fn validate(payload: &CreateUser) -> masterror::AppResult<()> {
    payload.validate().map_err(|err| {
        AppError::validation("invalid user payload")
            .with_code(AppCode::new("VALIDATION_ERROR"))
            .with_details(&err)
    })
}
```

`validator::ValidationErrors` implements `Serialize`, so it plugs directly into
`with_details`.

## Emitting HTTP responses manually

Sometimes you need to control the HTTP layer yourself (e.g., custom middleware).
Convert `AppError` into `ErrorResponse` and format it however you need.

```rust
fn to_json(err: &masterror::AppError) -> serde_json::Value {
    let response: masterror::ErrorResponse = err.clone().into();
    serde_json::json!({
        "status": response.status.as_u16(),
        "code": response.code,
        "message": response.message,
        "details": response.details,
    })
}
```

The clone is cheap because `AppError` uses shared references for heavy context
objects.

## Capturing reproducible logs

1. Log errors at the boundary with `tracing::error!`, including `kind`,
   `code`, and `retry` metadata.
2. Attach upstream errors via `with_context`. When you need additional metadata,
   derive your error type with fields annotated using `#[provide]` from
   `masterror::Error`.

```rust
#[tracing::instrument(skip(err))]
fn log_for_support(err: &masterror::AppError) {
    tracing::error!(
        kind = ?err.kind,
        code = ?err.code,
        retry = ?err.retry,
        auth = ?err.www_authenticate,
        "request failed",
    );
}
```

`#[tracing::instrument]` captures spans automatically, so support teams can
reconstruct what happened.

## Debugging common issues

| Symptom | Checklist |
|---------|-----------|
| Validation failures return HTTP 500 | Enable the `validator` feature and expose handlers as `AppResult<T>`. |
| JSON response lacks `code` | Call `.with_code(AppCode::new("..."))` or derive it via `#[app_error(code = ...)]`. |
| Logs show duplicated errors | Log once per request at the boundary; do not log again inside helpers. |
| `with_details` fails to compile | Ensure the value implements `Serialize` (derive or implement it manually). |
| Need to inspect nested errors | Call `err.context()` to retrieve captured sources, including `anyhow::Error`. |

## Testing strategies

- Unit-test constructors: assert on `AppErrorKind`, `AppCode`, retry hints, and
  JSON serialisation. Use `serde_json::to_value` for comparisons.
- Integration-test HTTP handlers: send requests using `axum::Router` or
  `actix_web::test::TestServer` and assert on status codes plus JSON bodies.
- Property-based tests (`proptest`) are great for validating validation logic and
  parsing code — ensure the error surfaces the expected code even for extreme
  inputs.

Keep tests deterministic and avoid network calls; use mocks or in-memory
services instead.
