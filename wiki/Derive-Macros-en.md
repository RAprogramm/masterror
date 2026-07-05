# Derive Macros

`masterror` ships two derives via the bundled `masterror-derive` crate:

- **`#[derive(Error)]`** — a drop-in replacement for `thiserror::Error` (same `#[error]`, `#[from]`, `#[source]`, `#[backtrace]` attributes) extended with `#[app_error(...)]` conversions and `#[provide(...)]` telemetry.
- **`#[derive(Masterror)]`** — builds on the same syntax and wires a domain error directly into `masterror::Error` with metadata, redaction policy and transport mapping tables via `#[masterror(...)]`.

Both are re-exported from the root: `use masterror::{Error, Masterror};`.

## `#[error("...")]` templates

The template drives the generated `Display` implementation. Placeholders reference fields by name (`{field}`), tuple index (`{0}`) or explicit arguments. Parsing is handled by the shared `masterror-template` crate and mirrors `thiserror` semantics.

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("{kind}: {message}")]
struct NamedError {
    kind:    &'static str,
    message: &'static str
}

#[derive(Debug, Error)]
#[error("{0} -> {1:?}")]
struct TupleError(&'static str, u8);
```

### Formatter traits and specs

Placeholders support the full formatter palette — `{x:?}`, `{x:#?}`, `{x:x}`, `{x:#X}`, `{x:b}`, `{x:o}`, `{x:e}`, `{x:E}`, `{x:p}` — and display-only specs such as `{value:>8}` or `{ratio:.3}` are forwarded verbatim. For programmatic template inspection, `masterror::error::template` exposes `ErrorTemplate`, `TemplateFormatter` and `TemplateFormatterKind`.

### Format arguments and projections

Templates accept named and positional arguments, including expressions on `self` and field projections with the `.field` shortcut:

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("{formatted}", formatted = self.message.to_uppercase())]
struct FormatArgExpressionError {
    message: &'static str
}

#[derive(Debug, Error)]
#[error("{}, {label}, {}", label = self.label, self.first, self.second)]
struct MixedImplicitArgsError {
    label:  &'static str,
    first:  &'static str,
    second: &'static str
}

#[derive(Debug, Error)]
#[error("{value}", value = .value)]
struct FieldShortcutError {
    value: &'static str
}
```

### `transparent` and `fmt = ...`

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("inner failure")]
struct Inner;

// Forwards Display and source() to the single wrapped field
#[derive(Debug, Error)]
#[error(transparent)]
struct Wrapper(#[from] Inner);

// Delegate rendering to a function: fields first, formatter last
fn render(count: &usize, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "count={count}")
}

#[derive(Debug, Error)]
#[error(fmt = crate::render)]
struct CustomFormat {
    count: usize
}
```

`transparent` requires exactly one field and cannot be combined with `fmt` or a template string. `fmt = path` points at a function receiving references to all fields plus the `Formatter`.

## Field attributes

| Attribute | Effect |
|---|---|
| `#[source]` | Field is returned from `source()`. `Option<E>` is supported. |
| `#[from]` | Generates `From<FieldType>` for the wrapper; implies `#[source]` on the same field. |
| `#[backtrace]` | Field holds a `std::backtrace::Backtrace` (or `Option<Backtrace>`) surfaced via error introspection, or delegates to the source's backtrace when combined with `#[source]`. |

Inference: a field literally named `source` is treated as the source automatically, and a field of type `std::backtrace::Backtrace` (or `Option<Backtrace>`) is picked up as the backtrace without an attribute.

Enums accept per-variant `#[error]` and per-variant `#[from]`/`#[source]`/`#[backtrace]`:

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("leaf failure")]
struct LeafError;

#[derive(Debug, Error)]
enum EnumError {
    #[error("unit failure")]
    Unit,
    #[error("{code}")]
    Code {
        code:  u16,
        #[source]
        cause: LeafError
    },
    #[error(transparent)]
    Wrapped(#[from] LeafError)
}
```

## `#[app_error(...)]` — conversions into AppError

Records how the derived error translates into `AppError`/`AppCode`. Options: `kind` (required), `code` (optional), `message` (flag).

```rust
use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("missing flag: {name}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
struct MissingFlag {
    name: &'static str
}

let app: AppError = MissingFlag { name: "feature" }.into();
assert!(matches!(app.kind, AppErrorKind::BadRequest));

let code: AppCode = MissingFlag { name: "other" }.into();
assert_eq!(code, AppCode::BadRequest);
```

- `kind = ...` selects the `AppErrorKind`; generates `From<T> for AppError`.
- `code = ...` additionally generates `From<T> for AppCode`.
- `message` forwards the `Display` output as the public message; omit it to keep the message internal.

Enums choose a mapping per variant, and the derive still emits a single `From<Enum> for AppError`.

## `#[provide(...)]` — typed telemetry

Exposes typed context through `std::error::Request` (nightly `error_generic_member_access`; compiled in automatically when available). `Option` fields only register a provider when populated:

```rust
use masterror::{AppCode, AppErrorKind, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
struct TelemetrySnapshot {
    name:  &'static str,
    value: u64
}

#[derive(Debug, Error)]
#[error("structured telemetry {snapshot:?}")]
#[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
struct StructuredTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    snapshot: TelemetrySnapshot
}
```

Consumers extract the snapshot with `std::error::request_ref::<TelemetrySnapshot>(&err)` on the domain error.

## `#[derive(Masterror)]` — end-to-end domain errors

`#[derive(Masterror)]` generates `Display`, `std::error::Error`, `From<T> for masterror::Error` **and** compile-time transport mapping tables, all configured by one `#[masterror(...)]` attribute:

```rust
use masterror::{
    AppCode, AppErrorKind, Error, Masterror, MessageEditPolicy, mapping::HttpMapping
};

#[derive(Debug, Masterror)]
#[error("user {user_id} missing flag {flag}")]
#[masterror(
    code = AppCode::NotFound,
    category = AppErrorKind::NotFound,
    message,
    redact(message, fields("user_id" = hash)),
    telemetry(
        Some(masterror::field::str("user_id", user_id.clone())),
        attempt.map(|value| masterror::field::u64("attempt", value))
    ),
    map.grpc = 5,
    map.problem = "https://errors.example.com/not-found"
)]
struct MissingFlag {
    user_id: String,
    flag:    &'static str,
    attempt: Option<u64>,
    #[source]
    source:  Option<std::io::Error>
}

let err = MissingFlag {
    user_id: "alice".into(),
    flag: "beta",
    attempt: Some(2),
    source: None
};
let converted: Error = err.into();
assert_eq!(converted.code, AppCode::NotFound);
assert_eq!(converted.kind, AppErrorKind::NotFound);
assert_eq!(converted.edit_policy, MessageEditPolicy::Redact);
assert!(converted.metadata().get("user_id").is_some());
assert_eq!(
    MissingFlag::HTTP_MAPPING,
    HttpMapping::new(AppCode::NotFound, AppErrorKind::NotFound)
);
```

### `#[masterror(...)]` options

| Option | Meaning |
|---|---|
| `code = AppCode::...` | Public machine-readable code |
| `category = AppErrorKind::...` | Semantic category (drives HTTP status) |
| `message` | Expose the formatted `Display` output as the safe public message |
| `redact(message)` | Set `MessageEditPolicy::Redact` so transports strip the message |
| `redact(fields("name" = hash, "card" = last4))` | Override per-field metadata policies: `hash`, `last4`, `redact`, `none` |
| `telemetry(expr, ...)` | Expressions evaluating to `Option<masterror::Field>`; populated fields are inserted into `Metadata`. Use `telemetry()` for none |
| `map.grpc = <i32>` | gRPC status code (matches `tonic::Code` discriminants) |
| `map.problem = "<uri>"` | RFC 7807 `type` URI |

### Generated mapping tables

For structs the derive emits associated constants; for enums it emits an array and slices aggregating the per-variant mappings:

| Shape | Constants |
|---|---|
| Struct | `T::HTTP_MAPPING: HttpMapping`, `T::GRPC_MAPPING: Option<GrpcMapping>`, `T::PROBLEM_MAPPING: Option<ProblemMapping>` |
| Enum | `T::HTTP_MAPPINGS: [HttpMapping; N]`, `T::GRPC_MAPPINGS: &'static [GrpcMapping]`, `T::PROBLEM_MAPPINGS: &'static [ProblemMapping]` |

The descriptor types live in `masterror::mapping` (`HttpMapping::status()` derives the HTTP code from the kind; `GrpcMapping::status()` returns the `i32`; `ProblemMapping::type_uri()` returns the URI).

`#[from]`, `#[source]` and `#[backtrace]` keep working under `#[derive(Masterror)]`; sources and captured backtraces are attached to the resulting `masterror::Error` automatically, and `Arc`-wrapped sources are reused without extra cloning.

## Choosing between the derives

| Need | Use |
|---|---|
| `Display` + `source` + `From`, thiserror-style | `#[derive(Error)]` |
| Also convert into `AppError`/`AppCode` | `#[derive(Error)]` + `#[app_error(...)]` |
| Typed context via `std::error::Request` | add `#[provide(...)]` |
| Metadata, redaction policy, gRPC/problem+json tables | `#[derive(Masterror)]` + `#[masterror(...)]` |

---

See also: [Getting Started](Getting-Started-en) · [Error Kinds and Codes](Error-Kinds-and-Codes-en) · [Context and Metadata](Context-and-Metadata-en) · [Migration](Migration-en)
