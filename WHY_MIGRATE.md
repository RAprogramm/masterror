<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Why Migrate to masterror in 2025

## TL;DR

**thiserror** and **anyhow** were great in 2021. It's 2025 now.

Modern production systems need:
- ✅ Structured observability (not just error messages)
- ✅ Transport-aware errors (HTTP/gRPC ready)
- ✅ Built-in redaction (GDPR/compliance by default)
- ✅ Typed metadata (not `HashMap<String, String>`)

**masterror provides all of this while maintaining API compatibility.**

## The Problems with thiserror/anyhow in 2025

### 1. thiserror: Feature-Complete but Frozen

```rust
// thiserror in 2025 - same as 2021
#[derive(Error, Debug)]
#[error("Database error: {0}")]
struct DbError(#[from] sqlx::Error);

// ❌ No structured metadata
// ❌ No HTTP status mapping
// ❌ No redaction policy
// ❌ No telemetry integration
// ❌ You implement this yourself, every time
```

**The reality:** Every production service adds the same boilerplate:
- Manual HTTP status code mapping
- Custom metadata extraction
- Homegrown redaction logic
- Manual tracing integration

**This is 2025. Stop reinventing the wheel.**

### 2. anyhow: Prototyping Tool, Not Production

```rust
// anyhow in production - convenient but limited
fn process() -> anyhow::Result<()> {
    do_thing().context("operation failed")?;
    Ok(())
}

// ❌ No type safety for errors
// ❌ Lost error information at boundaries
// ❌ No structured fields
// ❌ No automatic transport mapping
// ❌ String-based context only
```

**The reality:** `anyhow` is perfect for CLIs and scripts.

For production APIs serving millions of requests? **You need more.**

## masterror: Production-Ready Error Handling

### Full thiserror Compatibility + More

```rust
// Drop-in replacement for thiserror
use masterror::Error;

#[derive(Debug, Error)]
#[error("Database error: {0}")]
struct DbError(#[from] sqlx::Error);

// ✅ Same API as thiserror
// ✅ Plus: automatic AppError conversion
// ✅ Plus: HTTP/gRPC mappings available
```

**Migration effort: change one import line.**

### Full anyhow Ergonomics + Type Safety

```rust
// All the anyhow convenience, with types
use masterror::prelude::*;

fn process() -> AppResult<()> {
    ensure!(condition, AppError::bad_request("invalid input"));

    // Simple context (anyhow-style)
    database_call().context("db operation failed")?;

    // Or structured context with metadata
    database_call()
        .ctx(|| Context::new(AppErrorKind::Database)
            .with(field::str("table", "users"))
            .with(field::u64("attempt", retry_count))
        )?;

    fail!(AppError::internal("unrecoverable"));
}

// ✅ Same ergonomics as anyhow (.context(), .chain(), .downcast_ref())
// ✅ Plus: typed errors
// ✅ Plus: structured metadata
// ✅ Plus: automatic tracing
```

### What You Get That Others Don't Have

#### 1. Structured Metadata (Not Strings)

```rust
// thiserror/anyhow: lose context
let err = anyhow!("DB error: user={}, attempt={}", user_id, attempt);
// ❌ String formatting
// ❌ Lost type information
// ❌ Can't query/filter in observability tools

// masterror: structured context
let err = AppError::database("query failed")
    .with(field::str("user_id", user_id))
    .with(field::u64("attempt", attempt))
    .with(field::duration("elapsed", elapsed));
// ✅ Typed fields
// ✅ Queryable in logs/metrics
// ✅ Automatic JSON serialization
```

#### 2. Transport Mappings (Not Manual)

```rust
// thiserror/anyhow: manual mapping
impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let status = match self {
            MyError::NotFound => StatusCode::NOT_FOUND,
            MyError::Unauthorized => StatusCode::UNAUTHORIZED,
            // ... repeat for every error type
        };
        // ... manual JSON construction
    }
}

// masterror: automatic
#[derive(Debug, Masterror)]
#[masterror(
    category = AppErrorKind::NotFound,
    code = AppCode::NotFound,
    map.grpc = 5  // NOT_FOUND
)]
struct UserNotFound { id: String }

// ✅ HTTP 404 automatic
// ✅ gRPC NOT_FOUND automatic
// ✅ RFC7807 Problem JSON automatic
```

#### 3. Redaction (Not Manual)

```rust
// thiserror/anyhow: manual redaction
let err = format!("Auth failed: {}", token); // ❌ LEAKED SECRET

// masterror: policy-based
AppError::unauthorized("auth failed")
    .with(field::str("token", token))
    .redact_field("token", FieldRedaction::Hash)
    .redactable();
// ✅ Token hashed in logs
// ✅ Message redacted in client response
// ✅ GDPR/compliance ready
```

#### 4. Observability (Not DIY)

```rust
// thiserror/anyhow: manual instrumentation
tracing::error!(
    error = %err,
    user_id = user_id,
    attempt = attempt,
    "database error"
);
metrics::counter!("errors_total").increment(1);

// masterror: automatic
let err = AppError::database("query failed")
    .with(field::str("user_id", user_id))
    .with(field::u64("attempt", attempt));
// ✅ Automatic tracing event
// ✅ Automatic metrics increment
// ✅ Automatic backtrace capture
// ✅ Zero boilerplate
```

#### 5. Error Introspection (anyhow Parity)

```rust
// anyhow: type-safe error inspection
if let Some(io_err) = err.downcast_ref::<io::Error>() {
    match io_err.kind() {
        io::ErrorKind::NotFound => /* handle */,
        _ => /* other */
    }
}

// masterror: same API, works with AppError
use masterror::ResultExt;

match database_op().context("db failed") {
    Err(err) => {
        if let Some(io_err) = err.downcast_ref::<io::Error>() {
            // ✅ Type-safe downcasting
            // ✅ Inspect wrapped error sources
            // ✅ Full anyhow API compatibility
        }
    }
    Ok(val) => val
}
```

## Migration Guide

### From thiserror

**Step 1:** Update imports
```rust
// Before
use thiserror::Error;

// After
use masterror::Error;
```

**Step 2:** Enhance (optional, add when needed)
```rust
#[derive(Debug, Error)]
#[error("DB error")]
#[app_error(
    kind = AppErrorKind::Database,
    code = AppCode::Database
)]
struct DbError {
    #[from]
    source: sqlx::Error
}
```

**Done.** Your code still compiles. Add features incrementally.

### From anyhow

**Step 1:** Replace Result type
```rust
// Before
use anyhow::{Result, Context};

// After
use masterror::{AppResult, ResultExt};
```

**Step 2:** Update error construction
```rust
// Before
bail!("invalid input");

// After
fail!(AppError::bad_request("invalid input"));
```

**Step 3:** Keep using .context() (it just works!)
```rust
// Before (anyhow)
.context("db error")?

// After (masterror) - identical API
.context("db error")?

// Or use structured context for better observability
.ctx(|| Context::new(AppErrorKind::Database)
    .with(field::str("table", "users"))
)?
```

**Step 4:** Error introspection works the same
```rust
// anyhow API still works
if let Some(io_err) = err.downcast_ref::<io::Error>() {
    // handle specific error type
}
```

**Result:** Type-safe, structured, observable errors with zero API friction.

## Real-World Impact

### Before (thiserror + manual boilerplate)

```rust
#[derive(Error, Debug)]
#[error("User {user_id} not found")]
struct UserNotFound { user_id: String }

// Manual HTTP mapping
impl IntoResponse for UserNotFound {
    fn into_response(self) -> Response {
        (StatusCode::NOT_FOUND, Json(json!({
            "error": "not_found",
            "message": self.to_string(),
        }))).into_response()
    }
}

// Manual tracing
tracing::error!(user_id = %self.user_id, "user not found");

// Manual metrics
metrics::counter!("errors_total", "type" => "not_found").increment(1);
```

**Lines of code: ~20 per error type**

### After (masterror)

```rust
#[derive(Debug, Masterror)]
#[error("User {user_id} not found")]
#[masterror(
    code = AppCode::NotFound,
    category = AppErrorKind::NotFound,
    message,
    telemetry(Some(field::str("user_id", user_id.clone())))
)]
struct UserNotFound { user_id: String }
```

**Lines of code: 8**

**Savings: 60% less boilerplate, 100% more features**

## Performance

masterror is **competitive** with thiserror/anyhow:

| Operation | thiserror | anyhow | masterror |
|-----------|-----------|--------|-----------|
| Error creation | 34ns | 23ns | **30ns** |
| Root cause | N/A | 139ns | **141ns** |
| Display | 133ns | 74ns | **79ns** |

**Overhead is minimal** (~5-20ns) for significantly more functionality.

Binary size: 944KB (vs thiserror 32KB, anyhow 566KB)
- **Worth it:** Includes HTTP/gRPC/telemetry/redaction that you'd implement anyway

## The Bottom Line

### It's 2025. Your errors should:

1. ✅ **Map to transports** (HTTP/gRPC) automatically
2. ✅ **Include structured metadata** for observability
3. ✅ **Respect privacy** with built-in redaction
4. ✅ **Integrate with telemetry** out of the box
5. ✅ **Maintain type safety** across boundaries

**thiserror** gives you #5. You build #1-4 yourself.

**anyhow** gives you convenience. You lose #5 and build #1-4 yourself.

**masterror** gives you everything.

## Migration Timeline

- **Week 1:** Add masterror to dependencies
- **Week 2:** Migrate one service/module
- **Week 3:** Evaluate metrics (less code, better observability)
- **Week 4+:** Roll out to remaining services

**ROI:** Immediate reduction in boilerplate, improved observability, compliance-ready errors.

## Support & Resources

- 📚 [Full Documentation](https://docs.rs/masterror)
- 📊 [Benchmarks](BENCHMARKS.md)
- 🔧 **[Examples](examples/)** - See working code for:
  - [Basic Usage](examples/basic_usage.rs) - Core error handling patterns
  - [thiserror Compatibility](examples/derive_error.rs) - Drop-in replacement
  - [Structured Metadata](examples/structured_metadata.rs) - Typed fields vs strings
  - [Redaction](examples/redaction.rs) - GDPR-compliant privacy controls
- 💬 [GitHub Issues](https://github.com/RAprogramm/masterror/issues)

---

**Stop building the same error infrastructure over and over.**

**Start shipping features instead.**

Switch to masterror.
