# Error Kinds and Codes

Two types form the backbone of the taxonomy:

- **`AppErrorKind`** — the *internal*, semantic category of a failure. Small, stable, framework-agnostic. Controls the default HTTP status.
- **`AppCode`** — the *public*, machine-readable code exposed to clients as a SCREAMING_SNAKE_CASE string (e.g. `"NOT_FOUND"`). Part of the wire contract.

Every `AppError` carries both. `AppCode::from(kind)` gives the canonical 1:1 mapping, and `AppError::with_code(...)` overrides the public code without changing the category.

## AppErrorKind taxonomy

| Variant | Meaning | HTTP |
|---|---|---|
| `NotFound` | Resource does not exist or is not visible to the caller | 404 |
| `Validation` | Structured input failed validation | 422 |
| `Conflict` | State conflict (unique key violation, version mismatch) | 409 |
| `Unauthorized` | Authentication required or failed | 401 |
| `Forbidden` | Authenticated but not allowed | 403 |
| `NotImplemented` | Operation not supported by this deployment | 501 |
| `BadRequest` | Malformed request or missing parameters | 400 |
| `TelegramAuth` | Telegram authentication flow failed | 401 |
| `InvalidJwt` | JWT expired, malformed or has wrong signature/claims | 401 |
| `RateLimited` | Client exceeded rate limits or quota | 429 |
| `Timeout` | Operation did not complete in time | 504 |
| `Network` | Network-level error (DNS, connect, TLS) | 503 |
| `DependencyUnavailable` | External dependency down or degraded | 503 |
| `Internal` | Unexpected server-side failure | 500 |
| `Database` | Database failure (query, connection, migration) | 500 |
| `Service` | Generic service-layer/business-logic failure | 500 |
| `Config` | Missing or invalid configuration | 500 |
| `Turnkey` | Turnkey subsystem failure | 500 |
| `Serialization` | Failed to encode data | 500 |
| `Deserialization` | Failed to decode data | 500 |
| `ExternalApi` | Upstream API returned an error | 500 |
| `Queue` | Queue publish/consume/ack failure | 500 |
| `Cache` | Cache read/write/encoding failure | 500 |

```rust
use masterror::AppErrorKind;

let kind = AppErrorKind::NotFound;
assert_eq!(kind.http_status(), 404);        // always available, u16
assert_eq!(kind.label(), "Not found");      // human-readable title
// With the `axum` feature: kind.status_code() -> axum::http::StatusCode
```

Design rules baked into the mapping: infrastructure and I/O issues default to 5xx; `Unauthorized` (401) means authentication failed, `Forbidden` (403) means authentication succeeded but access was denied; use `Network` for connect/build failures and `ExternalApi` for upstream HTTP status errors.

## AppCode

`AppCode` ships constants matching every kind (`AppCode::NotFound` → `"NOT_FOUND"`, `AppCode::RateLimited` → `"RATE_LIMITED"`, …) plus `AppCode::UserAlreadyExists` (`"USER_ALREADY_EXISTS"`, mapped as a conflict). It is `#[non_exhaustive]` and supports caller-defined codes:

```rust
use std::str::FromStr;
use masterror::AppCode;

// Compile-time literal — panics at compile-time evaluation if not SCREAMING_SNAKE_CASE
const INVALID_JSON: AppCode = AppCode::new("INVALID_JSON");

// Runtime value — validated, returns Result<AppCode, ParseAppCodeError>
let dynamic = AppCode::try_new(String::from("THIRD_PARTY_FAILURE")).expect("valid code");
assert_eq!(dynamic.as_str(), "THIRD_PARTY_FAILURE");

// Parsing round-trips through the same validation
let parsed = AppCode::from_str("NOT_FOUND").expect("known code");
assert_eq!(parsed, AppCode::NotFound);
```

Valid codes contain only `A-Z`, `0-9` and single `_` separators, and serialize as plain JSON strings.

## HTTP / gRPC / problem+json mapping table

`CODE_MAPPINGS` (and the `mapping_for_code` lookup) define the canonical transport mapping for every built-in code. Unknown custom codes fall back to `INTERNAL` (500 / gRPC 13):

| AppCode | HTTP | gRPC | problem `type` |
|---|---|---|---|
| `NOT_FOUND` | 404 | `NOT_FOUND` (5) | `https://errors.masterror.rs/not-found` |
| `VALIDATION` | 422 | `INVALID_ARGUMENT` (3) | `.../validation` |
| `CONFLICT` | 409 | `ALREADY_EXISTS` (6) | `.../conflict` |
| `USER_ALREADY_EXISTS` | 409 | `ALREADY_EXISTS` (6) | `.../user-already-exists` |
| `UNAUTHORIZED` | 401 | `UNAUTHENTICATED` (16) | `.../unauthorized` |
| `FORBIDDEN` | 403 | `PERMISSION_DENIED` (7) | `.../forbidden` |
| `NOT_IMPLEMENTED` | 501 | `UNIMPLEMENTED` (12) | `.../not-implemented` |
| `BAD_REQUEST` | 400 | `INVALID_ARGUMENT` (3) | `.../bad-request` |
| `RATE_LIMITED` | 429 | `RESOURCE_EXHAUSTED` (8) | `.../rate-limited` |
| `TELEGRAM_AUTH` | 401 | `UNAUTHENTICATED` (16) | `.../telegram-auth` |
| `INVALID_JWT` | 401 | `UNAUTHENTICATED` (16) | `.../invalid-jwt` |
| `INTERNAL` | 500 | `INTERNAL` (13) | `.../internal` |
| `DATABASE` | 500 | `INTERNAL` (13) | `.../database` |
| `SERVICE` | 500 | `INTERNAL` (13) | `.../service` |
| `CONFIG` | 500 | `INTERNAL` (13) | `.../config` |
| `TURNKEY` | 500 | `INTERNAL` (13) | `.../turnkey` |
| `TIMEOUT` | 504 | `DEADLINE_EXCEEDED` (4) | `.../timeout` |
| `NETWORK` | 503 | `UNAVAILABLE` (14) | `.../network` |
| `DEPENDENCY_UNAVAILABLE` | 503 | `UNAVAILABLE` (14) | `.../dependency-unavailable` |
| `SERIALIZATION` | 500 | `INTERNAL` (13) | `.../serialization` |
| `DESERIALIZATION` | 500 | `INTERNAL` (13) | `.../deserialization` |
| `EXTERNAL_API` | 500 | `UNAVAILABLE` (14) | `.../external-api` |
| `QUEUE` | 500 | `UNAVAILABLE` (14) | `.../queue` |
| `CACHE` | 500 | `UNAVAILABLE` (14) | `.../cache` |

gRPC values match `tonic::Code` discriminants, so the `tonic` feature converts directly.

```rust
use masterror::{AppCode, mapping_for_code};

let mapping = mapping_for_code(&AppCode::Timeout);
assert_eq!(mapping.http_status(), 504);
assert_eq!(mapping.grpc().name, "DEADLINE_EXCEEDED");
assert_eq!(mapping.grpc().value, 4);
assert_eq!(mapping.problem_type(), "https://errors.masterror.rs/timeout");
```

## Retry and authentication hints

Transport adapters translate two optional hints into HTTP headers:

```rust
use std::time::Duration;
use masterror::{AppError, AppErrorKind, ProblemJson};

let problem = ProblemJson::from_app_error(
    AppError::new(AppErrorKind::Unauthorized, "Token expired")
        .with_retry_after_secs(30)
        .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#)
);

assert_eq!(problem.status, 401);
assert_eq!(problem.retry_after, Some(30));       // -> Retry-After header
assert!(problem.www_authenticate.is_some());     // -> WWW-Authenticate header
assert_eq!(problem.grpc.expect("grpc").name, "UNAUTHENTICATED");
```

On `ErrorResponse` the equivalent builders are `with_retry_after_secs`, `with_retry_after_duration` and `with_www_authenticate`.

## Redaction semantics

`AppError` messages are meant to be safe for clients, but you can mark an error as redactable so the boundary strips it:

```rust
use masterror::{AppError, MessageEditPolicy, ProblemJson};

let err = AppError::internal("host db-3 credentials rejected").redactable();
assert_eq!(err.edit_policy, MessageEditPolicy::Redact);

let problem = ProblemJson::from_app_error(err);
assert!(problem.detail.is_none());   // message stripped
assert!(problem.metadata.is_none()); // metadata stripped too
```

When `edit_policy` is `Redact`, `ProblemJson` drops `detail`, `details` and the entire `metadata` section. Individual metadata fields additionally carry their own `FieldRedaction` policy (`None`, `Redact`, `Hash`, `Last4`) applied during serialization — see [Context and Metadata](Context-and-Metadata-en). Error sources (`source_ref()`) are never serialized regardless of policy.

## Wire payloads

**`ProblemJson`** — RFC 7807 `application/problem+json`, produced by `ProblemJson::from_app_error` (owned) or `ProblemJson::from_ref` (borrowed). Fields: `type`, `title` (kind label), `status`, `detail`, optional `details`, `code`, `grpc` (`{name, value}`), `metadata`, plus non-serialized `retry_after`/`www_authenticate` for headers.

**`ErrorResponse`** — legacy flat JSON payload: `status`, `code`, `message`, optional `details`, `retry`, `www_authenticate`. With the `openapi` feature it derives `utoipa::ToSchema`.

```rust
use masterror::{AppCode, AppError, AppErrorKind, ErrorResponse};

let app_err = AppError::new(AppErrorKind::NotFound, "user_not_found");
let resp: ErrorResponse = (&app_err).into();
assert_eq!(resp.status, 404);
assert_eq!(resp.code, AppCode::NotFound);
```

Prefer `ProblemJson` for new APIs; `ErrorResponse` remains for services already committed to the flat shape.

---

See also: [Getting Started](Getting-Started-en) · [Derive Macros](Derive-Macros-en) · [Context and Metadata](Context-and-Metadata-en) · [Web Frameworks](Web-Frameworks-en)
