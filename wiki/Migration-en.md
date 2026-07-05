# Migration

`masterror` is designed as a drop-in successor to both `thiserror` (derive
syntax) and `anyhow` (ergonomics). Most migrations are a dependency swap plus
incremental adoption of the typed features. Runnable walkthroughs:
[`examples/migrate_from_thiserror.rs`](https://github.com/RAprogramm/masterror/blob/main/examples/migrate_from_thiserror.rs)
and
[`examples/migrate_from_anyhow.rs`](https://github.com/RAprogramm/masterror/blob/main/examples/migrate_from_anyhow.rs).

## From thiserror

Step 1 is mechanical — change the import; the derive syntax is compatible:

```diff
-use thiserror::Error;
+use masterror::Error;
```

### Attribute compatibility

| thiserror | masterror | Notes |
|---|---|---|
| `#[error("...")]` with `{field}` placeholders | same | 1:1, including positional `{0}` |
| Format specs `:>8`, `:.3`, `:x`, `:p`, `:e` | same | `TemplateFormatter` mirrors thiserror's formatter detection |
| `#[error(transparent)]` | same | enforces single-field wrappers forwarding `Display`/`source` |
| `#[from]` | same | generates `From<...>`, validates wrapper shape |
| `#[source]` | same | wires the `source()` chain |
| `#[backtrace]` | same | honoured on fields |
| — | `#[app_error(kind = ..., code = ..., message)]` | **added**: generates `From<YourError> for AppError` (and `AppCode`); `message` forwards `Display` as the public message |
| — | `#[provide(ref = T, value = T)]` | **added**: typed telemetry via `std::error::Request`; `Option<T>` fields provide only when `Some` |
| — | `#[derive(Masterror)]` + `#[masterror(...)]` | **added**: full mapping with `category`, `redact(message, fields(...))`, `telemetry(...)`, `map.grpc`, `map.problem` |

Existing enums keep compiling unchanged. What you gain by annotating them:

```rust
use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("user {user_id} not found")]
#[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound, message)]
struct UserMissing {
    user_id: u64
}

let app: AppError = UserMissing { user_id: 42 }.into();
assert_eq!(app.kind, AppErrorKind::NotFound);
```

Enums map per variant — each variant carries its own `#[app_error(...)]`, and
the derive emits a single `From<Enum> for AppError`.

Recommended order: (1) swap the import, (2) add `#[app_error]` to types that
cross the API boundary, (3) replace hand-written `From<DomainError> for
AppError` impls with the generated ones, (4) adopt `#[masterror(...)]` where
you need redaction or metadata.

## From anyhow

| anyhow | masterror | Notes |
|---|---|---|
| `anyhow::Result<T>` | `masterror::AppResult<T>` | alias for `Result<T, AppError>` |
| `anyhow::Error` | `masterror::AppError` / `Error` | carries kind, code, metadata instead of a blob |
| `.context("msg")` | `.context("msg")` | identical, via `masterror::ResultExt` |
| `.with_context(\|\| ...)` | `.ctx(\|\| Context::new(kind)...)` | lazy like anyhow, but builds a **typed** `Context` (kind, code, fields, redaction) |
| `bail!(err)` | `fail!(err)` | takes a typed error expression, no format machinery |
| `ensure!(cond, "msg {x}")` | `ensure!(cond, AppError::...)` | condition + typed error; no formatting on the success path |
| `err.chain()` | `err.chain()` | same iterator over the source chain |
| `err.root_cause()` | `err.root_cause()` | same |
| `err.is::<E>()` / `downcast` / `downcast_ref` / `downcast_mut` | same names on `AppError` | downcasting parity |
| `#[from]`-style wrapping | `From<...>` impls behind [feature flags](Integrations-en) | sqlx/redis/reqwest/... arrive pre-classified |

`.context()` works exactly as you expect:

```rust
use masterror::{AppResult, ResultExt};

fn read_config(path: &str) -> AppResult<String> {
    let content = std::fs::read_to_string(path).context("Failed to read config file")?;
    Ok(content)
}
```

`ensure!`/`fail!` trade anyhow's string formatting for typed errors:

```rust
use masterror::{AppError, AppResult, ensure, fail, field};

fn parse(content: &str, path: &str) -> AppResult<()> {
    ensure!(
        !content.is_empty(),
        AppError::bad_request("Config file is empty")
            .with_field(field::str("path", path.to_owned()))
    );
    if content.starts_with("invalid") {
        fail!(AppError::bad_request("Invalid config format"));
    }
    Ok(())
}
```

The error expression is evaluated only when the guard trips, so the happy path
stays allocation-free — same guarantee anyhow gives, plus a machine-readable
code.

### Where anyhow has no equivalent

Migrating buys you capabilities that have no anyhow counterpart:

- **Typed taxonomy** — `AppErrorKind` (internal) and `AppCode` (public,
  SCREAMING_SNAKE_CASE) instead of stringly-typed context. See
  [Error Kinds & Codes](Error-Kinds-and-Codes-en).
- **Transport mappings** — RFC 7807 `problem+json` for Axum/Actix and
  `tonic::Status` for gRPC, derived from the same code table. See
  [Web Frameworks](Web-Frameworks-en).
- **Telemetry** — automatic `tracing` events, `error_total{code,category}`
  metrics and lazy backtraces at the boundary. See
  [Observability](Observability-en).
- **Redaction** — `redactable()` messages and per-field `Hash`/`Last4`/`Redact`
  policies, honoured by every transport. See
  [Best Practices](Best-Practices-en).
- **Structured metadata** — typed `field::str/u64/duration/ip/...` instead of
  formatting values into the message.

### What to watch for

- anyhow's `ensure!(cond, "format {}", x)` formatted-message form has no
  direct twin: construct the error explicitly
  (`AppError::bad_request(format!(...))`) or, better, use a static message
  plus metadata fields.
- `anyhow::Error` accepts any `E: Error + Send + Sync`. In masterror you
  choose a kind at wrap time (`Context::new(kind)` or a `From` conversion) —
  that decision point is the feature, not friction: it is where
  classification happens.
- Both `ensure!` and `fail!` expand to `return Err(...)`, so they work in any
  function returning `Result<_, E>` where your expression is already the
  error type — no `Into` conversion is inserted.

See also: [Getting Started](Getting-Started-en) · [Derive Macros](Derive-Macros-en) · [Context & Metadata](Context-and-Metadata-en) · [Best Practices](Best-Practices-en)
