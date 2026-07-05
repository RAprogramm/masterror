# Best Practices

Patterns that keep `masterror`-based services predictable: typed domain
errors, one stable code taxonomy, transport mapping at the edge, and public
messages that never leak internals.

## Derive domain errors, map them once

Model each bounded context as an enum with `#[derive(Error)]` and declare the
`AppError` mapping inline with `#[app_error(...)]`. The derive generates
`Display`, `From<...>` for wrapped sources, and the conversion into
`AppError`/`AppCode` — no hand-written `match` at every call site.

```rust
use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
pub enum UserError {
    #[error("user {0} not found")]
    #[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound, message)]
    NotFound(u64),

    #[error("email already registered")]
    #[app_error(kind = AppErrorKind::Conflict, code = AppCode::Conflict, message)]
    DuplicateEmail,

    #[error("storage failure")]
    #[app_error(kind = AppErrorKind::Database, code = AppCode::Database)]
    Storage(#[from] std::io::Error)
}

let app: AppError = UserError::DuplicateEmail.into();
assert_eq!(app.kind, AppErrorKind::Conflict);
```

Include `message` only on variants whose `Display` output is safe to show to
clients. Omit it (as on `Storage`) to keep the text internal — the client sees
only the kind's title. Full attribute reference:
[Derive Macros](Derive-Macros-en).

## One `AppCode` taxonomy per service

`AppCode` is your public API contract; clients branch on it. Keep the set
small, documented and per-service:

- Prefer the built-in codes (`NOT_FOUND`, `CONFLICT`, `VALIDATION`, ...) —
  they already carry canonical HTTP/gRPC/problem-type mappings.
- Mint custom codes centrally, not inline at call sites:

```rust
use masterror::AppCode;

pub const CODE_PLAN_LIMIT: AppCode = AppCode::new("PLAN_LIMIT_EXCEEDED");
```

`AppCode::new` is `const` and panics at compile time on anything that is not
SCREAMING_SNAKE_CASE; use `AppCode::try_new` for runtime strings. Renaming a
code is a breaking API change — treat additions like adding an enum variant.

## Map to transports at the edge only

Domain and service layers return `AppResult<T>` and know nothing about HTTP.
The single `IntoResponse`/`ResponseError`/`Status` implementation in the crate
does the mapping in the handler layer:

```rust,ignore
async fn get_user(id: u64, repo: &Repo) -> masterror::AppResult<User> {
    repo.find(id).await?          // sqlx::Error -> AppError::NotFound/Database
}
```

Never hand-construct status codes in business logic and never implement a
second response conversion — the stable `AppErrorKind → status` table in
[Web Frameworks](Web-Frameworks-en) is the one source of truth.

## Redact sensitive data, keep telemetry

Two independent knobs:

- **Message redaction** — `err.redactable()` (or `redact(message)` in
  `#[masterror(...)]`) hides `detail` from wire payloads while logs keep it.
- **Field redaction** — per-field policy applied when metadata is serialized:

```rust
use masterror::{AppError, FieldRedaction, field};

let err = AppError::bad_request("Invalid credentials")
    .with_field(field::str("email", "user@example.com").with_redaction(FieldRedaction::Hash))
    .with_field(field::str("card", "4111111111111111").with_redaction(FieldRedaction::Last4))
    .with_field(field::str("ip", "192.168.1.100").with_redaction(FieldRedaction::Redact));
```

`Hash` keeps correlational value (same input → same digest) without exposing
the raw string; `Last4` suits card/token suffixes; `Redact` removes the value
entirely. Default is `None`. Redact anything user-identifying by default and
opt out consciously, not the other way around.

## `Context` vs derive

- **Derive** when the error type is part of your domain vocabulary: it has
  variants, appears in signatures, and its mapping is static.
- **`Context`** (via `ResultExt::ctx`) when wrapping an infrastructure error
  ad hoc at a call site and the classification depends on the operation, not
  the type:

```rust
# #[cfg(feature = "std")] {
use masterror::{AppErrorKind, Context, ResultExt, field};

fn read_state() -> masterror::AppResult<Vec<u8>> {
    std::fs::read("/var/lib/app/state.bin").ctx(|| {
        Context::new(AppErrorKind::Internal)
            .with(field::str("path", "/var/lib/app/state.bin"))
            .track_caller()
    })
}
# }
```

`ctx` is lazy — the closure runs only on the error path. Use plain
`.context("message")` when a human-readable note is all you need. Details:
[Context & Metadata](Context-and-Metadata-en).

## Test kinds and codes, not strings

Assert on the stable taxonomy, never on formatted messages:

```rust
use masterror::{AppCode, AppError, AppErrorKind, ProblemJson};

let err = AppError::not_found("user 42 missing");
assert_eq!(err.kind, AppErrorKind::NotFound);
assert_eq!(err.code, AppCode::NotFound);

let problem = ProblemJson::from_ref(&err);
assert_eq!(problem.status, 404);
assert_eq!(problem.code.as_str(), "NOT_FOUND");
```

- `ProblemJson::from_ref` lets integration tests assert the exact wire
  contract without spinning up a server.
- `mapping_for_code(&code)` exposes the canonical HTTP status, gRPC code and
  problem-type URI for table-driven tests.
- For redaction tests, assert `problem.detail.is_none()` on a `redactable()`
  error and check `metadata().iter_with_redaction()` policies.

## Public message vs internal telemetry

A useful rule for every error you construct:

| Channel | Contents |
|---|---|
| `message` / `detail` | Human-oriented, non-sensitive, stable enough to show a user |
| `Metadata` fields | IDs, attempts, endpoints, durations — for logs/metrics, with redaction policies |
| `source` chain | Raw underlying errors — logged, **never** serialized to clients |

`masterror` enforces the last row (sources are never written to wire
payloads), but the first two are your responsibility: if a string contains
anything you would not print in a browser, put it in metadata with a
redaction policy or mark the error `redactable()`.

See also: [Derive Macros](Derive-Macros-en) · [Context & Metadata](Context-and-Metadata-en) · [Error Kinds & Codes](Error-Kinds-and-Codes-en) · [Migration](Migration-en)
