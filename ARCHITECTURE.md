<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Architecture Documentation

## Table of Contents

1. [High-Level Design](#high-level-design)
2. [Module Structure](#module-structure)
3. [Core Types](#core-types)
4. [Data Flow](#data-flow)
5. [Extension Points](#extension-points)
6. [Performance Characteristics](#performance-characteristics)
7. [Design Patterns](#design-patterns)
8. [Compile-Time Guarantees](#compile-time-guarantees)

## High-Level Design

### Design Philosophy

masterror follows a **layered architecture** with clear separation between:

1. **Core layer**: Framework-agnostic error types and metadata
2. **Conversion layer**: Integration with third-party libraries
3. **Transport layer**: HTTP, gRPC, and serialization adapters
4. **Derive layer**: Procedural macros for ergonomic derivation
5. **Turnkey layer**: Opinionated defaults for rapid adoption

```
┌─────────────────────────────────────────────────────────┐
│                    Application Code                      │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Turnkey Layer (Optional)                    │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Pre-built catalog, classifiers, helper functions │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                   Derive Layer                           │
│  ┌────────────────┬─────────────────┬─────────────┐    │
│  │ #[derive(Error)]│ #[derive(Master-│ #[provide]  │    │
│  │                 │      ror)]      │             │    │
│  └────────────────┴─────────────────┴─────────────┘    │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                 Transport Layer                          │
│  ┌──────────┬───────────┬─────────┬──────────────┐     │
│  │  Axum    │  Actix    │  Tonic  │  OpenAPI     │     │
│  │ Responder│ Responder │ Status  │ Schema Gen   │     │
│  └──────────┴───────────┴─────────┴──────────────┘     │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│               Conversion Layer                           │
│  ┌───────┬────────┬───────┬────────┬─────────────┐     │
│  │ sqlx  │reqwest │ redis │tokio   │ validator   │     │
│  │       │        │       │        │ ...         │     │
│  └───────┴────────┴───────┴────────┴─────────────┘     │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                    Core Layer                            │
│  ┌──────────────────────────────────────────────────┐  │
│  │ AppError │ AppErrorKind │ AppCode │ Metadata     │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Workspace Organization

```
masterror/
├── masterror/              # Main crate (re-exports all layers)
├── masterror-derive/       # Procedural macros
└── masterror-template/     # Template parser (used by derive)
```

**Dependency direction**: `masterror` → `masterror-derive` → `masterror-template`

Derive macros expand to code that uses `masterror` public API, creating a build-time dependency cycle handled by Cargo's macro expansion phase.

## Module Structure

### Core Modules

#### `src/lib.rs`
Entry point, re-exports public API, manages feature flags.

#### `src/app_error/`
Core error type implementation.

- **`core.rs`**: `AppError` struct definition and `std::error::Error` impl
- **`constructors.rs`**: Convenience constructors (`AppError::internal`, `::not_found`, etc.)
- **`metadata.rs`**: `Metadata` and `Field` types for structured context
- **`context.rs`**: `Context` builder for attaching metadata

#### `src/kind.rs`
`AppErrorKind` enum mapping to HTTP 4xx/5xx classes and internal categories.

```rust
pub enum AppErrorKind {
    BadRequest,      // 400
    Unauthorized,    // 401
    Forbidden,       // 403
    NotFound,        // 404
    Conflict,        // 409
    Timeout,         // 408
    Service,         // 502/503
    Internal,        // 500
    // ...
}
```

#### `src/code.rs` and `src/code/app_code.rs`
`AppCode` enum for fine-grained classification (100+ variants).

```rust
pub enum AppCode {
    BadRequest,
    InvalidFormat,
    MissingField,
    Unauthorized,
    TokenExpired,
    // ...
}
```

#### `src/mapping.rs`
Transport mapping definitions (`HttpMapping`, `GrpcMapping`, `ProblemMapping`).

### Conversion Layer

#### `src/convert/`
Third-party error conversions via `From` trait impls.

- **`sqlx.rs`**: Database errors → `AppErrorKind::Conflict`, `::Service`, `::Internal`
- **`reqwest.rs`**: HTTP client errors → `AppErrorKind::Service`, `::Timeout`
- **`redis.rs`**: Redis errors → `AppErrorKind::Service`, `::Internal`
- **`tokio.rs`**: Async runtime errors → `AppErrorKind::Internal`, `::Timeout`
- **`validator.rs`**: Validation errors → `AppErrorKind::BadRequest`
- **`config.rs`**: Configuration errors → `AppErrorKind::Internal`
- **`teloxide.rs`**: Telegram bot errors → `AppErrorKind::Service`, `::BadRequest`
- **`multipart.rs`**: Multipart form errors → `AppErrorKind::BadRequest`

Each conversion:
1. Maps error variant to `AppErrorKind`
2. Preserves source error chain
3. Attaches relevant telemetry (e.g., SQL constraint name, HTTP status code)

### Transport Layer

#### `src/response/`
HTTP and serialization adapters.

- **`core.rs`**: `ErrorResponse` trait for framework-agnostic responses
- **`problem_json.rs`**: RFC 7807 Problem Details serialization
- **`axum_impl.rs`**: `impl IntoResponse for AppError` (Axum)
- **`actix_impl.rs`**: `impl ResponseError for AppError` (Actix-web)
- **`mapping.rs`**: HTTP status code and gRPC status mappings
- **`metadata.rs`**: Metadata serialization with redaction

#### `src/convert/tonic.rs`
`impl From<AppError> for tonic::Status` with gRPC code mapping.

#### `src/convert/axum.rs` and `src/convert/actix.rs`
Extended Axum/Actix integrations beyond basic `IntoResponse`.

### Derive Macros

Located in `masterror-derive/src/`:

- **`error_derive.rs`**: `#[derive(Error)]` implementation
- **`error_trait.rs`**: Trait generation and method synthesis
- **`masterror_derive.rs`**: `#[derive(Masterror)]` with telemetry and redaction
- **`provide_derive.rs`**: `#[provide]` for `std::error::Request` providers

### Turnkey Module

#### `src/turnkey/`
Opinionated defaults for rapid adoption.

- **`domain.rs`**: Pre-built domain error types
- **`classifier.rs`**: Error classification heuristics
- **`conversions.rs`**: Automatic conversions with telemetry

### Utility Modules

#### `src/macros.rs`
`ensure!` and `fail!` macros for control flow.

#### `src/prelude.rs`
Commonly used types for glob imports.

#### `src/result_ext.rs`
`ResultExt` trait for ergonomic error context attachment.

#### `src/frontend/`
WASM/browser compatibility.

- **`browser_console_error.rs`**: `console.error()` integration
- **`browser_console_ext.rs`**: Trait extensions for browser logging

## Core Types

### `AppError`

```rust
pub struct AppError {
    pub kind: AppErrorKind,
    pub code: AppCode,
    pub message: String,
    pub edit_policy: MessageEditPolicy,
    metadata: Metadata,
    source: Option<Box<dyn Error + Send + Sync>>,
    backtrace: Option<Backtrace>,
    retry_after: Option<RetryAfter>,
    www_authenticate: Option<String>,
}
```

**Invariants**:
- `kind` and `code` must be semantically consistent (enforced by constructors)
- `source` chain is immutable after construction
- `metadata` is append-only (fields can be added but not removed)
- `message` is either user-facing or internal based on `edit_policy`

### `Metadata`

```rust
pub struct Metadata {
    fields: Vec<Field>,
}

pub enum Field {
    Str { key: &'static str, value: String, policy: RedactionPolicy },
    I64 { key: &'static str, value: i64, policy: RedactionPolicy },
    U64 { key: &'static str, value: u64, policy: RedactionPolicy },
    F64 { key: &'static str, value: f64, policy: RedactionPolicy },
    Duration { key: &'static str, value: Duration, policy: RedactionPolicy },
    IpAddr { key: &'static str, value: IpAddr, policy: RedactionPolicy },
    Json { key: &'static str, value: Value, policy: RedactionPolicy },
}
```

**Invariants**:
- Keys are static strings (zero allocation overhead)
- Fields are ordered by insertion
- Redaction policy is immutable per field
- No duplicate keys (last insert wins)

### `Context`

Builder for attaching metadata:

```rust
pub struct Context {
    kind: AppErrorKind,
    code: AppCode,
    message: String,
    fields: Vec<Field>,
}
```

**Usage**:
```rust
AppError::internal("db error")
    .with_field(field::str("table", "users"))
    .with_field(field::duration("query_time", elapsed))
```

## Data Flow

### Error Creation Flow

```
User Code
  │
  ├─ AppError::new(kind, message)
  │    │
  │    └─> AppError { kind, code: default(kind), message, ... }
  │
  └─ AppError::internal(message).with_field(...)
       │
       └─> Context::new(kind, message)
            │
            └─> Context::with_field(field)
                 │
                 └─> Context::into_error()
                      │
                      └─> AppError { ..., metadata }
```

### Third-Party Error Conversion Flow

```
sqlx::Error
  │
  └─ From<sqlx::Error> for AppError
      │
      ├─ match error_variant:
      │   ├─ Database constraint → AppErrorKind::Conflict
      │   ├─ Connection error → AppErrorKind::Service
      │   └─ Query error → AppErrorKind::Internal
      │
      ├─ Extract metadata (table, constraint, etc.)
      │
      └─> AppError {
            kind: ...,
            code: ...,
            message: ...,
            source: Some(sqlx_error),
            metadata: [...]
          }
```

### HTTP Response Flow

```
AppError
  │
  └─ axum::IntoResponse::into_response()
      │
      ├─ ProblemJson::from_app_error()
      │   │
      │   ├─ Map kind → HTTP status
      │   ├─ Serialize metadata (apply redaction)
      │   └─> RFC 7807 JSON payload
      │
      ├─ Build headers
      │   ├─ Content-Type: application/problem+json
      │   ├─ Retry-After: ... (if set)
      │   └─ WWW-Authenticate: ... (if set)
      │
      └─> axum::Response
```

### Derive Macro Expansion Flow

```
#[derive(Error)]
#[error("db error: {source}")]
struct DbError {
    #[source]
    source: io::Error,
}

  │ (proc macro expansion)
  ▼

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "db error: {}", self.source)
    }
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

## Extension Points

### 1. Custom Error Types

Implement `From<CustomError> for AppError`:

```rust
impl From<MyDomainError> for AppError {
    fn from(err: MyDomainError) -> Self {
        match err {
            MyDomainError::NotFound(id) => {
                AppError::not_found("resource missing")
                    .with_field(field::str("resource_id", id))
            }
            MyDomainError::InvalidInput(msg) => {
                AppError::bad_request(msg)
            }
        }
    }
}
```

### 2. Custom Metadata Fields

Define domain-specific field builders:

```rust
pub mod field {
    pub fn user_id(value: impl Into<String>) -> Field {
        Field::str_redacted("user_id", value, RedactionPolicy::Hash)
    }

    pub fn transaction_amount(cents: i64) -> Field {
        Field::i64("transaction_cents", cents, RedactionPolicy::None)
    }
}
```

### 3. Custom Transport Mappings

Override default mappings via derive attributes:

```rust
#[derive(Masterror)]
#[masterror(
    category = AppErrorKind::Service,
    map.grpc = 14,  // UNAVAILABLE
    map.problem = "https://api.example.com/errors/db-unavailable"
)]
struct DatabaseUnavailable;
```

### 4. Custom Redaction Policies

Implement `RedactionPolicy` trait:

```rust
impl RedactionPolicy {
    pub fn custom_mask(&self, value: &str) -> String {
        match self {
            RedactionPolicy::Custom => mask_pii(value),
            _ => self.apply_default(value),
        }
    }
}
```

### 5. Custom Telemetry Providers

Implement `std::error::Request` providers:

```rust
#[derive(Error)]
#[error("telemetry snapshot")]
struct MyError {
    #[provide(ref = TelemetrySnapshot)]
    snapshot: TelemetrySnapshot,
}
```

## Performance Characteristics

### Allocation Patterns

**Zero-allocation paths**:
- Error kind/code classification
- Static message errors (`AppError::internal("static")`)
- Metadata field key storage (uses `&'static str`)

**Single-allocation paths**:
- Dynamic message errors (allocates `String`)
- Metadata field value storage (one allocation per field)

**Multiple-allocation paths**:
- Source error boxing (unavoidable for trait objects)
- Backtrace capture (when enabled)
- RFC 7807 JSON serialization

### Time Complexity

- `AppError::new()`: **O(1)**
- `with_field()`: **O(1)** amortized (Vec::push)
- `From<ThirdPartyError>`: **O(1)** to **O(k)** where k = number of fields attached
- `ProblemJson::from_app_error()`: **O(n)** where n = number of metadata fields
- HTTP response generation: **O(n)** for serialization

### Memory Layout

```
AppError: 120 bytes (on x86_64)
├─ kind: 1 byte (enum discriminant)
├─ code: 2 bytes (enum discriminant)
├─ message: 24 bytes (String)
├─ edit_policy: 1 byte (enum discriminant)
├─ metadata: 24 bytes (Vec<Field>)
├─ source: 16 bytes (Option<Box<...>>)
├─ backtrace: 16 bytes (Option<Backtrace>)
├─ retry_after: 16 bytes (Option<RetryAfter>)
└─ www_authenticate: 24 bytes (Option<String>)
```

Field: 40-48 bytes depending on variant (unoptimized enum layout).

### Benchmark Results

Typical performance on modern x86_64 CPU:

- **Error creation with metadata**: ~50-100ns
- **Context into_error conversion**: ~80-150ns
- **ProblemJson serialization**: ~300-500ns
- **Full HTTP response generation**: ~800-1200ns

Regressions >10% from these baselines fail CI.

## Design Patterns

### 1. Builder Pattern

`Context` and `AppError` fluent APIs:

```rust
AppError::internal("db error")
    .with_field(field::str("table", "users"))
    .with_retry_after_duration(Duration::from_secs(30))
```

### 2. Strategy Pattern

Redaction policies encapsulate field masking strategies:

```rust
enum RedactionPolicy {
    None,
    Redact,
    Hash,
    Last4,
}
```

### 3. Adapter Pattern

Transport layers adapt `AppError` to framework-specific types:

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Adapt to Axum's Response type
    }
}
```

### 4. Type-State Pattern

`Context` ensures errors are constructed correctly before conversion:

```rust
Context::new(kind, message)  // Incomplete state
    .with_field(field)        // Still building
    .into_error()             // Transition to complete state
```

### 5. Extension Trait Pattern

`ResultExt` adds methods to standard `Result`:

```rust
pub trait ResultExt<T, E> {
    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce(E) -> AppError;
}
```

### 6. Derive Macro Pattern

Procedural macros generate boilerplate implementations:

```rust
#[derive(Error)]  // Generates Display, Error, From impls
```

## Compile-Time Guarantees

### 1. No Unsafe Code

Enforced by:
```toml
[lints.rust]
unsafe_code = "forbid"
```

### 2. Type Safety

- Error kinds are enum variants (no string matching)
- Metadata fields are strongly typed (no `HashMap<String, String>`)
- Source errors preserve type information via trait objects

### 3. Send + Sync Bounds

All errors implement `Send + Sync` for concurrent usage:

```rust
impl Error for AppError + Send + Sync + 'static
```

### 4. No Panics in Library Code

Enforced by code review and testing. Only `unreachable!()` after exhaustive matches.

### 5. Feature Flag Isolation

Feature combinations tested in CI via matrix:

```yaml
features: [
  ["std"],
  ["std", "axum"],
  ["std", "actix"],
  ["std", "turnkey"],
  ["std", "axum", "sqlx", "tracing"],
]
```

### 6. MSRV Guarantees

CI tests on MSRV (1.90) and stable to prevent accidental newer Rust usage.

## Deployment Considerations

### Feature Flag Selection

**Minimal setup** (library use):
```toml
masterror = { version = "0.24", default-features = false }
```

**HTTP service**:
```toml
masterror = { version = "0.24", features = ["std", "axum", "tracing"] }
```

**Full-stack service**:
```toml
masterror = { version = "0.24", features = [
    "std", "axum", "sqlx", "reqwest", "redis",
    "tracing", "metrics", "backtrace"
] }
```

**Rapid prototype**:
```toml
masterror = { version = "0.24", features = ["turnkey"] }
```

### Error Budget

Typical error overhead per request:
- **CPU**: 100-200ns error creation + 500-1000ns serialization
- **Memory**: 120 bytes base + 40 bytes per metadata field
- **Allocations**: 1-3 allocations per error (depending on metadata)

For high-throughput services (>100k req/s), consider:
- Reusing error instances via thread-local storage
- Limiting metadata fields to <5 per error
- Disabling backtrace capture in production

### Observability Integration

**Tracing**:
```rust
#[instrument(err)]
fn operation() -> Result<T, AppError> {
    // Errors automatically logged with span context
}
```

**Metrics**:
```rust
let counter = error_counter(err.kind, err.code);
counter.increment(1);
```

## Future Architecture Evolution

### Planned Enhancements

1. **Async context propagation**: Store metadata in tokio task-local storage
2. **OpenTelemetry native**: Directly export errors as OTel events
3. **Error aggregation**: Batch errors for distributed tracing
4. **Recovery strategies**: Optional retry/fallback builders

### Stability Guarantees

- **Core types**: Stable, semver-compatible
- **Transport adapters**: Semver-minor for new adapters
- **Derive macros**: Syntax is stable, expansion may improve
- **Turnkey module**: May evolve with breaking changes (opt-in)

### Deprecation Policy

- Deprecated features remain for 2 minor versions
- Migration guides provided in CHANGELOG
- Compiler warnings guide users to replacements
