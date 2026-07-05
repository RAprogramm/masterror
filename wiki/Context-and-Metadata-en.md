# Context and Metadata

`masterror` replaces string-glued context (`format!("failed to X: {e}")`) with three structured mechanisms: the `Context` builder, typed `Metadata` fields, and redaction policies enforced at the transport boundary.

## ResultExt: promoting foreign errors

`ResultExt` is implemented for every `Result<T, E>` where `E: Error + Send + Sync + 'static` and offers two methods:

### `.context(msg)` — anyhow-style

Wraps the error with a message; the original error becomes the source:

```rust
use masterror::ResultExt;

fn read_config() -> Result<String, std::io::Error> {
    Err(std::io::Error::from(std::io::ErrorKind::NotFound))
}

let err = read_config().context("Failed to read config file").unwrap_err();
assert!(err.source_ref().is_some());
```

If the underlying error is already a `masterror::Error`, `.context()` preserves its classification: kind, code, metadata, edit policy, retry advice and details are carried over, only the message is replaced and the original error is kept as the source.

### `.ctx(|| Context)` — full control

```rust
use masterror::{AppErrorKind, Context, ResultExt, field};

fn validate() -> Result<(), std::io::Error> {
    Err(std::io::Error::other("boom"))
}

let err = validate()
    .ctx(|| {
        Context::new(AppErrorKind::Validation)
            .with(field::str("phase", "validate"))
            .redact(true)
            .track_caller()
    })
    .unwrap_err();

assert_eq!(err.kind, AppErrorKind::Validation);
assert!(err.metadata().get("phase").is_some());
```

The closure is only evaluated on the error path.

## The Context builder

| Method | Effect |
|---|---|
| `Context::new(kind)` | Target category; the `AppCode` defaults to the canonical mapping for that kind |
| `.code(AppCode)` | Override the public code |
| `.category(kind)` | Change the category; keeps the code in sync unless it was overridden |
| `.with(field)` | Attach a metadata `Field` |
| `.redact(bool)` | Toggle message redaction (`MessageEditPolicy::Redact` / `Preserve`) |
| `.redact_field(name, FieldRedaction)` | Override the redaction policy of a named field |
| `.track_caller()` | Record the call site as `caller.file`, `caller.line`, `caller.column` metadata |

## Metadata fields

`Metadata` is a sorted, inline-allocated map of typed fields (0–4 fields stay on the stack). Build fields with the `masterror::field` module:

| Builder | `FieldValue` variant |
|---|---|
| `field::str("key", value)` | `Str(Cow<'static, str>)` |
| `field::i64("key", -1)` | `I64` |
| `field::u64("key", 42)` | `U64` |
| `field::f64("key", 0.5)` | `F64` |
| `field::bool("key", true)` | `Bool` |
| `field::uuid("key", uuid)` | `Uuid` |
| `field::duration("key", dur)` | `Duration` |
| `field::ip("key", addr)` | `Ip` (v4 or v6) |
| `field::json("key", json!({...}))` | `Json` (requires `serde_json` feature) |

Attach fields when constructing errors or through `Context`:

```rust
use core::time::Duration;
use masterror::{AppError, FieldValue, field};

let err = AppError::service("downstream degraded")
    .with_field(field::str("request_id", "abc123"))
    .with_field(field::duration("elapsed", Duration::from_millis(1500)))
    .with_field(field::u64("attempt", 2));

assert_eq!(err.metadata().len(), 3);
assert_eq!(err.metadata().get("attempt"), Some(&FieldValue::U64(2)));

for (name, value) in err.metadata().iter() {
    println!("{name}={value}");
}
```

`with_fields(iter)` extends from an iterator, `with_metadata(meta)` replaces the container, and `Metadata::insert` returns the previous value when a key is overwritten.

## Redaction policies

### Message policy: `MessageEditPolicy`

`Preserve` (default) keeps the public message; `Redact` tells transports to strip it. Set it with `.redactable()` on an error, `.redact(true)` on a `Context`, or `redact(message)` in `#[masterror(...)]`:

```rust
use masterror::{AppError, MessageEditPolicy, ProblemJson};

let err = AppError::internal("db-3 credentials rejected").redactable();
assert_eq!(err.edit_policy, MessageEditPolicy::Redact);

let problem = ProblemJson::from_app_error(err);
assert!(problem.detail.is_none());
```

### Field policy: `FieldRedaction`

Each field carries its own policy applied when metadata is serialized into `ProblemJson`:

| Policy | Effect on the public payload |
|---|---|
| `None` | Value preserved as-is |
| `Redact` | Field removed entirely |
| `Hash` | Value replaced with a SHA-256 digest |
| `Last4` | All but the last four characters masked |

```rust
use masterror::{AppError, FieldRedaction, field};

let err = AppError::internal("payment failed")
    .with_field(field::str("card_number", "4111111111111111"))
    .redact_field("card_number", FieldRedaction::Last4);
```

Common secret-like names get a safe default automatically when the field is created: names containing `password`, `secret`, `authorization`, `cookie`, `session`, `jwt`, `bearer`, `otp`, `pin` default to `Redact`; token/key-like names (`api_token`, `refresh_token`, `key`, `apikey`) default to `Hash`; card/account segments combined with a number-like segment (`card_number`, `iban_no`, `account_id`) default to `Last4`. Detection is case-insensitive. Explicit `redact_field`/`with_redaction` always wins.

## Error chains

Errors keep their full causal chain. `chain()` iterates from the error itself down to the root cause; `root_cause()` jumps straight to the deepest error:

```rust
use masterror::AppError;

let io_err = std::io::Error::other("disk offline");
let app_err = AppError::internal("db down").with_context(io_err);

let chain: Vec<_> = app_err.chain().collect();
assert_eq!(chain.len(), 2);
assert_eq!(app_err.root_cause().to_string(), "disk offline");
```

`with_context(...)` is the preferred way to attach an upstream error: it accepts owned errors or shared `Arc<dyn Error + Send + Sync>` values and reuses existing allocations. `with_source(...)` / `with_source_arc(...)` are the lower-level equivalents.

## Downcasting

Inspect the attached source with `is` and `downcast_ref`/`downcast_mut`:

```rust
use masterror::AppError;

let io_err = std::io::Error::other("disk offline");
let err = AppError::internal("boom").with_context(io_err);

assert!(err.is::<std::io::Error>());

if let Some(io) = err.downcast_ref::<std::io::Error>() {
    assert_eq!(io.to_string(), "disk offline");
}
```

- `is::<E>()` — `true` when the immediate source is of type `E` (does not walk the whole chain).
- `downcast_ref::<E>()` — borrow the source as `E`.
- `downcast::<E>()` / `downcast_mut::<E>()` — currently stubs (`downcast` always returns `Err(self)`, `downcast_mut` always returns `None`), so prefer `downcast_ref`.

For deeper matches, walk `chain()` and use `source.is::<E>()` / `source.downcast_ref::<E>()` on each element.

## Backtraces

With the `backtrace` feature, `err.backtrace()` returns a lazily captured `std::backtrace::Backtrace` honouring `RUST_BACKTRACE`, and `with_backtrace(bt)` attaches an explicit capture. Backtraces are shared via `Arc` when errors are re-wrapped, so `.context()` chains do not re-capture.

---

See also: [Getting Started](Getting-Started-en) · [Error Kinds and Codes](Error-Kinds-and-Codes-en) · [Derive Macros](Derive-Macros-en) · [Observability](Observability-en) · [Best Practices](Best-Practices-en)
