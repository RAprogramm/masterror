<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Requirements Specification

> This specification follows the [RustManifest](https://github.com/RAprogramm/RustManifest) development standards, which define professional guidelines for Rust project structure, code quality, testing, and documentation.

## 1. Problem Statement

### 1.1 Context

Modern Rust services require consistent, observable error handling across multiple transport layers (HTTP, gRPC, WebAssembly) and integration boundaries (databases, HTTP clients, message queues). Existing error handling libraries present several challenges:

1. **Fragmentation**: `thiserror` provides derives but lacks transport mappings; `anyhow` offers dynamic errors but loses type safety; combining both creates inconsistent error surfaces.

2. **Transport overhead**: Mapping domain errors to HTTP status codes, gRPC status values, and RFC 7807 Problem Details requires repetitive glue code in every service.

3. **Telemetry gaps**: Structured error context (request IDs, user IDs, operation metadata) is typically stored as strings in ad-hoc maps, losing type safety and making redaction policies difficult to enforce.

4. **Integration burden**: Each third-party library error (sqlx, reqwest, redis) needs manual conversion logic, source chain preservation, and context attachment.

5. **Framework coupling**: Error types often become tightly coupled to web frameworks (Actix, Axum), making code reuse across projects difficult.

### 1.2 Target Users

1. **Backend service developers** building microservices with multiple transport protocols
2. **Library authors** requiring framework-agnostic error types with rich context
3. **Platform teams** establishing error handling standards across service fleets
4. **WebAssembly developers** needing browser-compatible error reporting without heap allocation requirements

### 1.3 Goals

1. Provide a unified error taxonomy that maps cleanly to HTTP, gRPC, and RFC 7807 Problem Details
2. Offer typed, structured telemetry with field-level redaction controls
3. Enable framework-agnostic error handling with optional transport adapters
4. Deliver zero-cost abstractions with no unsafe code
5. Maintain comprehensive test coverage and documentation

### 1.4 Non-Goals

1. Replace all uses of `thiserror` or `anyhow` in existing codebases (compatibility is preferred)
2. Provide dynamic error type erasure by default (users can opt into this via features)
3. Offer automatic error recovery or retry logic (this is application-specific)
4. Support runtime error message localization (compile-time message selection is possible)

## 2. Use Cases

### 2.1 UC-001: HTTP Service Error Handling

**Actor**: Backend service developer
**Precondition**: Service uses Axum or Actix-web
**Trigger**: Business logic returns domain error
**Main Flow**:
1. Domain function returns `Result<T, DomainError>`
2. `DomainError` implements `From<DomainError> for AppError`
3. Framework automatically converts `AppError` to HTTP response
4. Client receives RFC 7807 Problem Details with appropriate status code

**Postcondition**: Error is logged with structured telemetry, client receives consistent error format

**Alternative Flows**:
- A1: If error contains sensitive data, redaction policy masks fields before serialization
- A2: If retry hint is set, `Retry-After` header is included in response

### 2.2 UC-002: gRPC Status Mapping

**Actor**: gRPC service developer
**Precondition**: Service uses Tonic
**Trigger**: Business logic returns domain error
**Main Flow**:
1. Domain function returns `Result<T, DomainError>`
2. `DomainError` defines gRPC status code mapping via derive macro
3. Error is converted to `tonic::Status` with message and details
4. gRPC client receives status with structured metadata

**Postcondition**: Error maintains consistent mapping between HTTP and gRPC transports

### 2.3 UC-003: Database Error Integration

**Actor**: Service using sqlx
**Precondition**: Database query fails
**Trigger**: sqlx returns `sqlx::Error`
**Main Flow**:
1. sqlx operation returns error
2. `From<sqlx::Error> for AppError` converts error with telemetry
3. Constraint violations map to `AppErrorKind::Conflict`
4. Connection errors map to `AppErrorKind::Service`
5. Query-specific metadata (table name, operation) is attached

**Postcondition**: Database errors are consistently classified and observable

### 2.4 UC-004: Structured Telemetry Attachment

**Actor**: Developer instrumenting error paths
**Precondition**: Operation has contextual metadata
**Trigger**: Error occurs during operation
**Main Flow**:
1. Developer creates error with metadata builders
2. Metadata includes typed fields (strings, integers, durations, IPs)
3. Fields have individual redaction policies (hash, last4, redact, none)
4. Error propagates through call stack preserving metadata
5. Telemetry backend receives structured fields

**Postcondition**: Errors are observable without string concatenation or manual JSON building

### 2.5 UC-005: Custom Domain Error Derivation

**Actor**: Library author defining domain errors
**Precondition**: Domain has specific error taxonomy
**Trigger**: Need to integrate domain errors with masterror
**Main Flow**:
1. Define domain error enum with variants
2. Apply `#[derive(Error, Masterror)]` with per-variant mappings
3. Specify `AppCode`, `AppErrorKind`, telemetry, and redaction per variant
4. Framework automatically converts to HTTP/gRPC responses
5. Source errors and backtraces are preserved

**Postcondition**: Domain errors integrate seamlessly without manual glue code

## 3. Functional Requirements

### 3.1 Core Error Types

**FR-001**: System SHALL provide `AppError` type with kind, code, message, metadata, and source
**FR-002**: System SHALL provide `AppErrorKind` enum covering HTTP 4xx/5xx status classes
**FR-003**: System SHALL provide `AppCode` enum for fine-grained error classification
**FR-004**: System SHALL support error source chaining via `std::error::Error`
**FR-005**: System SHALL support backtrace capture when enabled

### 3.2 Metadata System

**FR-006**: System SHALL support typed metadata fields (string, i64, u64, f64, Duration, IpAddr, JSON)
**FR-007**: System SHALL provide field-level redaction policies (none, hash, last4, redact)
**FR-008**: System SHALL provide builders for common field types
**FR-009**: System SHALL allow metadata attachment at error creation time
**FR-010**: System SHALL preserve metadata through error conversions

### 3.3 Derive Macros

**FR-011**: System SHALL provide `#[derive(Error)]` compatible with thiserror syntax
**FR-012**: System SHALL provide `#[derive(Masterror)]` for full integration
**FR-013**: System SHALL support per-variant mappings in enum errors
**FR-014**: System SHALL generate `From` implementations for source errors
**FR-015**: System SHALL support telemetry attachment via derive attributes

### 3.4 Transport Mappings

**FR-016**: System SHALL map `AppErrorKind` to HTTP status codes
**FR-017**: System SHALL map `AppErrorKind` to gRPC status codes
**FR-018**: System SHALL generate RFC 7807 Problem Details JSON
**FR-019**: System SHALL support custom Problem type URIs
**FR-020**: System SHALL include retry hints in HTTP headers when specified

### 3.5 Integrations

**FR-021**: System SHALL provide `From` implementations for sqlx errors
**FR-022**: System SHALL provide `From` implementations for reqwest errors
**FR-023**: System SHALL provide `From` implementations for redis errors
**FR-024**: System SHALL provide `From` implementations for validator errors
**FR-025**: System SHALL provide `From` implementations for config errors
**FR-026**: System SHALL provide `From` implementations for tokio errors
**FR-027**: System SHALL provide `From` implementations for teloxide errors
**FR-028**: System SHALL provide `From` implementations for multipart errors

### 3.6 Feature Flags

**FR-029**: System SHALL compile with no default features enabled
**FR-030**: System SHALL provide opt-in features for each integration
**FR-031**: System SHALL provide opt-in features for each transport
**FR-032**: System SHALL provide opt-in features for telemetry backends
**FR-033**: System SHALL provide turnkey feature with opinionated defaults

### 3.7 Documentation

**FR-034**: System SHALL document all public types with doc comments
**FR-035**: System SHALL provide doctests for all public APIs
**FR-036**: System SHALL maintain comprehensive README with examples
**FR-037**: System SHALL publish documentation on docs.rs
**FR-038**: System SHALL maintain CHANGELOG with migration notes

## 4. Non-Functional Requirements

### 4.1 Performance

**NFR-001**: Error creation SHALL NOT allocate on the happy path
**NFR-002**: Metadata attachment SHALL use `Vec::with_capacity` for known sizes
**NFR-003**: Error conversions SHALL complete in <100ns for common paths (measured via criterion)
**NFR-004**: RFC 7807 serialization SHALL complete in <500ns for typical errors
**NFR-005**: Performance regressions >10% SHALL fail CI benchmarks

### 4.2 Memory Safety

**NFR-006**: System SHALL contain zero unsafe code
**NFR-007**: System SHALL pass Miri tests under stacked borrows
**NFR-008**: System SHALL prevent memory leaks (validated via leak sanitizer)
**NFR-009**: System SHALL use only safe abstractions from std and alloc

### 4.3 Code Quality

**NFR-010**: System SHALL maintain >95% code coverage (target: 100%)
**NFR-011**: System SHALL pass clippy with `-D warnings`
**NFR-012**: System SHALL format code with `cargo +nightly fmt`
**NFR-013**: System SHALL pass cargo-deny license and advisory checks
**NFR-014**: System SHALL pass cargo-audit security vulnerability checks

### 4.4 Compatibility

**NFR-015**: System SHALL support MSRV 1.90
**NFR-016**: System SHALL support Rust edition 2024
**NFR-017**: System SHALL support no_std environments (with alloc)
**NFR-018**: System SHALL support WASM targets (with frontend feature)
**NFR-019**: System SHALL maintain semver compatibility guarantees

### 4.5 Build Characteristics

**NFR-020**: Core crate SHALL compile in <30s on release profile
**NFR-021**: Full feature set SHALL compile in <60s on release profile
**NFR-022**: Default feature set SHALL have <10 direct dependencies
**NFR-023**: Release builds SHALL enable LTO and single codegen unit
**NFR-024**: Documentation builds SHALL complete without warnings

### 4.6 Testing

**NFR-025**: System SHALL include unit tests for all functions
**NFR-026**: System SHALL include integration tests for all transport adapters
**NFR-027**: System SHALL include doctests for all public APIs
**NFR-028**: System SHALL test error paths, not just happy paths
**NFR-029**: System SHALL test concurrent error handling (no data races)
**NFR-030**: System SHALL test resource cleanup (no file/socket leaks)

### 4.7 CI/CD

**NFR-031**: CI SHALL run on every PR and main branch push
**NFR-032**: CI SHALL test MSRV and stable Rust in matrix
**NFR-033**: CI SHALL run fmt, clippy, tests, coverage, benchmarks, audit
**NFR-034**: CI SHALL fail on any warnings or test failures
**NFR-035**: CI SHALL upload coverage reports to Codecov
**NFR-036**: CI SHALL cancel redundant runs via concurrency groups

### 4.8 Documentation

**NFR-037**: Every public type SHALL have doc comments with examples
**NFR-038**: Every public function SHALL document parameters and return values
**NFR-039**: Complex algorithms SHALL have explanatory comments
**NFR-040**: Migration guides SHALL document breaking changes
**NFR-041**: Architecture decisions SHALL be documented in ADRs

## 5. Architecture Decision Records

### 5.1 ADR-001: Framework-Agnostic Core

**Status**: Accepted
**Context**: Need to support multiple web frameworks and non-HTTP use cases
**Decision**: Core types in separate crate with optional transport adapters via features
**Consequences**:
- Positive: Reusable across projects, no framework lock-in
- Positive: Smaller compile times for users not needing all transports
- Negative: Requires feature flag management
- Negative: Transport impls must be maintained separately

### 5.2 ADR-002: Typed Metadata Over String Maps

**Status**: Accepted
**Context**: Need structured telemetry without losing type safety
**Decision**: Provide typed field builders (i64, u64, Duration, IpAddr) with per-field redaction
**Consequences**:
- Positive: Type-safe context attachment
- Positive: Granular redaction control
- Positive: Better telemetry backend integration
- Negative: More complex API than simple HashMap<String, String>
- Negative: Serialization logic must handle multiple types

### 5.3 ADR-003: Native Derive Macros

**Status**: Accepted
**Context**: Need to integrate domain errors without manual glue code
**Decision**: Implement derive macros compatible with thiserror syntax plus masterror extensions
**Consequences**:
- Positive: Seamless migration from thiserror
- Positive: Reduced boilerplate
- Positive: Compile-time validation of mappings
- Negative: Proc macro complexity
- Negative: Error messages from macros can be cryptic

### 5.4 ADR-004: Conservative HTTP/gRPC Mappings

**Status**: Accepted
**Context**: Need consistent error classification across transports
**Decision**: Provide default mappings with override capability
**Consequences**:
- Positive: Consistent out-of-box behavior
- Positive: Reduces mapping errors
- Positive: Teams can customize per domain
- Negative: Defaults may not fit all use cases
- Negative: Requires documentation of mapping rationale

### 5.5 ADR-005: No Unsafe Code

**Status**: Accepted
**Context**: Target WASM and security-critical services
**Decision**: Implement all functionality with safe Rust abstractions
**Consequences**:
- Positive: Memory safety guarantees
- Positive: WASM compatibility
- Positive: Easier to audit
- Negative: May lose some performance optimizations
- Negative: Cannot use certain low-level tricks

### 5.6 ADR-006: Opt-In Feature Flags

**Status**: Accepted
**Context**: Avoid forcing dependencies on users who don't need them
**Decision**: All integrations and transports behind opt-in features, no defaults
**Consequences**:
- Positive: Minimal dependency footprint
- Positive: Faster compile times for minimal use cases
- Positive: Users only pay for what they use
- Negative: Users must explicitly enable features
- Negative: Documentation must explain feature combinations

### 5.7 ADR-007: Turnkey Module for Rapid Adoption

**Status**: Accepted
**Context**: Teams want opinionated defaults for quick onboarding
**Decision**: Provide optional turnkey module with pre-built catalog and helpers
**Consequences**:
- Positive: Faster time to value
- Positive: Consistent baseline across teams
- Positive: Can still customize after adoption
- Negative: Adds maintenance burden
- Negative: May not fit all domain models

### 5.8 ADR-008: MSRV Policy

**Status**: Accepted
**Context**: Balance modern Rust features with ecosystem compatibility
**Decision**: MSRV 1.90 with edition 2024 support
**Consequences**:
- Positive: Access to latest stable features
- Positive: Future-proof for edition migration
- Negative: Excludes older Rust installations
- Mitigation: MSRV bumps are semver-minor per Cargo policy

## 6. Design Tradeoffs

### 6.1 Type Safety vs Ergonomics

**Choice**: Typed metadata fields over `HashMap<String, String>`
**Rationale**: Compile-time type checking prevents runtime errors; redaction policies require field types
**Alternative**: Generic string maps (used by many logging libraries)
**Impact**: Slightly more verbose API, but safer and more observable

### 6.2 Feature Flags vs Monolithic Crate

**Choice**: Extensive feature flags with no defaults
**Rationale**: Minimize compile time and dependency footprint for minimal use cases
**Alternative**: All-in-one crate with all features enabled
**Impact**: Users must read docs to enable features, but gain significant compile time improvements

### 6.3 thiserror Compatibility vs Novel Syntax

**Choice**: Derive macros compatible with thiserror syntax
**Rationale**: Lower migration barrier, familiar to Rust developers
**Alternative**: Completely novel derive syntax
**Impact**: Constrained by thiserror design choices, but easier adoption

### 6.4 Conservative Defaults vs Domain Flexibility

**Choice**: Opinionated HTTP/gRPC mappings with override capability
**Rationale**: Consistency out of box, but allow customization
**Alternative**: No default mappings, require explicit configuration
**Impact**: Works immediately for 80% of use cases, customizable for edge cases

### 6.5 Zero Unsafe vs Performance

**Choice**: No unsafe code, even if it costs some performance
**Rationale**: Memory safety and WASM compatibility trump marginal perf gains
**Alternative**: Targeted unsafe for hot paths
**Impact**: May be slightly slower than hand-optimized alternatives, but safer

## 7. Future Considerations

### 7.1 Potential Enhancements

1. **Async context propagation**: Integrate with tokio tracing spans
2. **Error recovery strategies**: Optional retry/fallback builders
3. **Metric integration**: Automatic error counters per kind/code
4. **OpenTelemetry**: Native span/event export
5. **Custom transports**: GraphQL, Protobuf, MessagePack adapters

### 7.2 Known Limitations

1. **Static messages**: Error messages are not localized at runtime
2. **No error recovery**: Library does not provide retry/circuit breaker logic
3. **Sync-first design**: Async-specific patterns (like tokio task cancellation) require manual integration
4. **JSON-only Problem Details**: XML RFC 7807 not currently supported

### 7.3 Maintenance Commitments

1. **MSRV updates**: Review quarterly, bump only for compelling features
2. **Dependency updates**: Monitor via Dependabot, apply security patches within 48 hours
3. **Breaking changes**: Follow semver strictly, provide migration guides
4. **Feature requests**: Prioritize framework-agnostic features over framework-specific

## 8. Acceptance Criteria

A release is considered ready when:

1. All functional requirements (FR-001 through FR-038) are implemented
2. All non-functional requirements (NFR-001 through NFR-041) are verified
3. Code coverage is >95% (target: 100%)
4. CI passes on MSRV and stable Rust
5. Documentation builds without warnings
6. Benchmarks show no regressions >10%
7. Manual testing validates all transport adapters
8. CHANGELOG documents all user-facing changes
9. Migration guide exists for breaking changes
10. Security audit passes (cargo-audit, cargo-deny)
