# Observability

`masterror` treats telemetry as part of the error lifecycle. Each `AppError`
tracks a dirty flag; telemetry is emitted once per state change — at
construction, after mutation, or when the error crosses a transport boundary
(Axum `IntoResponse`, Actix `error_response()`, tonic `Status` conversion).
You rarely call anything manually.

## Feature flags

| Feature | Adds |
|---|---|
| `tracing` | Structured `tracing` event per error, `trace_id` via `log-mdc` |
| `metrics` | `error_total{code,category}` counter via the `metrics` crate |
| `backtrace` | Lazy `std::backtrace::Backtrace` capture gated by `RUST_BACKTRACE` |
| `colored` | ANSI-colored terminal styling with TTY detection |

```toml
[dependencies]
masterror = { version = "0.28", features = ["tracing", "metrics", "backtrace"] }
```

## Tracing

With `tracing` enabled, each error emits one ERROR-level event with target
`masterror::error`:

| Field | Content |
|---|---|
| `code` | `AppCode` string, e.g. `NOT_FOUND` |
| `category` | `AppErrorKind` label, e.g. `Database` |
| `message` | Public message, if any |
| `retry_seconds` | Retry advice, if set |
| `redactable` | Whether the message is redacted at transport boundaries |
| `metadata_len` | Number of attached metadata fields |
| `www_authenticate` | Authentication challenge, if set |
| `trace_id` | Pulled from the `log-mdc` context key `trace_id`, if present |

The emission is subscriber-aware: if no subscriber is interested in
ERROR-level events for the target, the event stays pending and is retried on
the next flush, so nothing is lost when a subscriber is installed late.

To correlate errors with requests, store a trace ID in the MDC in your request
middleware:

```rust,ignore
log_mdc::insert("trace_id", request_id);
```

Every error constructed while the key is set carries it in the event.

## Metrics

With `metrics` enabled, each newly-dirty error increments:

```text
error_total{code="NOT_FOUND", category="NotFound"}
```

Both labels are stable strings (`AppCode::as_str()` and the `AppErrorKind`
label), so dashboards and alerts survive refactors of your domain types. Wire
any `metrics` recorder (Prometheus, StatsD, ...) as usual; `masterror` only
uses `metrics::counter!`.

## Backtraces

With `backtrace` enabled, a `Backtrace` snapshot is captured lazily when
telemetry is flushed — not on every construction. Capture is controlled by
`RUST_BACKTRACE`: unset, empty, `0`, `off` and `false` disable it; anything
else enables it. The preference is read once and cached per process.

```rust
# #[cfg(feature = "backtrace")] {
use masterror::AppError;

let err = AppError::internal("db down");
if let Some(bt) = err.backtrace() {
    eprintln!("{bt}");
}
# }
```

You can also attach a pre-captured trace with
`AppError::with_backtrace(backtrace)`, which takes priority over lazy capture.

## Manual flushing with `.log()`

Constructors and conversions emit telemetry automatically. After mutating an
error (adding fields, changing retry advice) you can force re-emission:

```rust
use masterror::{AppError, field};

let err = AppError::service("upstream degraded")
    .with_field(field::str("upstream", "billing"));
err.log();
```

`log()` is idempotent per state: if nothing changed since the last emission,
it does nothing. The HTTP/gRPC adapters flush the same way at the boundary,
so an error that is constructed, enriched and then returned from an Axum
handler emits once per state — once at construction and once at the boundary
for the enriched state — never twice for the same state.

## Inspecting the chain

Independent of features, `AppError` exposes the tools log pipelines need:

```rust
# #[cfg(feature = "std")] {
use std::io::Error as IoError;
use masterror::AppError;

let err = AppError::internal("db down").with_context(IoError::other("disk offline"));

assert_eq!(err.chain().count(), 2);
let _root = err.root_cause();
assert!(err.metadata().is_empty());
# }
```

`metadata().iter_with_redaction()` yields `(key, value, policy)` triples so a
logging layer can honour field redaction — see
[Context & Metadata](Context-and-Metadata-en).

## Colored terminal output

The `colored` feature adds `masterror::colored::style` for CLI tools. Colors
are applied only when stderr is a TTY, `NO_COLOR` is unset, `TERM` is not
`dumb` and the terminal supports ANSI; otherwise the text passes through
unchanged. Detection is cached per process.

| Function | Style | Use for |
|---|---|---|
| `error_kind_critical` | red | critical failure kinds |
| `error_kind_warning` | yellow | recoverable/warning kinds |
| `error_code` | cyan | machine codes |
| `error_message` | bright white | main message |
| `source_context` | dimmed | secondary/source info |
| `metadata_key` | green | structured field names |

```rust
# #[cfg(feature = "colored")] {
use masterror::colored::style;

eprintln!(
    "{}: {}",
    style::error_code("ERR_DB_001"),
    style::error_message("Database connection failed")
);
# }
```

See the runnable demo in
[`examples/colored_cli.rs`](https://github.com/RAprogramm/masterror/blob/main/examples/colored_cli.rs).

See also: [Feature Flags](Feature-Flags-en) · [Context & Metadata](Context-and-Metadata-en) · [Web Frameworks](Web-Frameworks-en) · [Best Practices](Best-Practices-en)
