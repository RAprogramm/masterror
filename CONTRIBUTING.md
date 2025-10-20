<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Contributing to masterror

Thank you for considering contributing to masterror. This document outlines the development process, coding standards, and quality requirements.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Environment](#development-environment)
3. [Coding Standards](#coding-standards)
4. [Testing Requirements](#testing-requirements)
5. [Pull Request Process](#pull-request-process)
6. [Commit Message Format](#commit-message-format)
7. [Code Review Guidelines](#code-review-guidelines)
8. [Performance Benchmarking](#performance-benchmarking)
9. [Documentation Standards](#documentation-standards)
10. [Issue Reporting](#issue-reporting)

## Getting Started

### Prerequisites

- **Rust**: MSRV 1.90 (stable and nightly toolchains recommended)
- **Tools**: `cargo-audit`, `cargo-deny`, `cargo-tarpaulin` or `cargo-llvm-cov` (for coverage)
- **GitHub CLI**: `gh` (optional but recommended for workflow management)

### Initial Setup

```bash
git clone https://github.com/RAprogramm/masterror.git
cd masterror

rustup toolchain install stable nightly
rustup component add clippy rustfmt --toolchain nightly

cargo build --all-features
cargo test --all-features
```

### Workspace Structure

```
masterror/
├── src/                    # Core library
├── masterror-derive/       # Procedural macros
├── masterror-template/     # Template parser
├── tests/                  # Integration tests
├── benches/                # Criterion benchmarks
├── examples/               # Usage examples
├── docs/                   # Extended documentation
└── .github/                # CI/CD workflows
```

## Development Environment

### Editor Configuration

Configure your editor to:
- Use 4 spaces for indentation
- Maximum line length: 99 characters
- Trim trailing whitespace
- Insert final newline

### Recommended Extensions

**VS Code**:
- rust-analyzer
- Even Better TOML
- crates

**IntelliJ IDEA**:
- Rust plugin

## Coding Standards

This project adheres to the [RustManifest](https://github.com/RAprogramm/RustManifest) guidelines, which define comprehensive standards for professional Rust development.

### Style Guide

We follow Rust standard formatting with project-specific overrides in `.rustfmt.toml` based on [RustManifest .rustfmt.toml reference](https://github.com/RAprogramm/RustManifest/blob/main/.rustfmt.toml):

```bash
cargo +nightly fmt --all
```

**Key principles**:
- Line limit: 99 characters
- Imports: grouped (std, external, internal)
- No `::` operator except in `use` statements
- Avoid `unwrap()` and `expect()` in library code (tests are exempt)
- Minimize `clone()` usage
- Meaningful variable names; constants in `SCREAMING_SNAKE_CASE`

### Documentation Requirements

Every public item SHALL have:

1. **Summary**: One-line description
2. **Details**: Behavior explanation
3. **Parameters**: Document all parameters
4. **Return value**: Document return semantics
5. **Errors**: Document error conditions
6. **Examples**: At least one doctest
7. **Safety**: Document unsafe code (if any)

Example:

```rust
/// Converts a domain error into an application error with telemetry.
///
/// This function attaches structured metadata fields to the error context,
/// applies the specified redaction policy, and maps the error kind to the
/// appropriate transport-level representation.
///
/// # Parameters
///
/// - `source`: The source error to convert
/// - `metadata`: Structured telemetry fields to attach
///
/// # Returns
///
/// Returns an `AppError` with the source error, metadata, and appropriate
/// error kind classification.
///
/// # Examples
///
/// ```
/// use masterror::{AppError, field};
///
/// let err = AppError::internal("db connection lost")
///     .with_field(field::str("table", "users"));
/// assert_eq!(err.metadata().len(), 1);
/// ```
pub fn with_field(self, field: Field) -> Self {
    // implementation
}
```

### Error Handling

- All fallible operations return `Result<T, E>`
- Use `masterror::AppError` for application-level errors
- Preserve error source chains
- Attach relevant context via metadata
- No panics except in tests and `unreachable!()` after exhaustive matches

### Type System Usage

- Prefer `&str` over `String` when ownership is not required
- Use `&[T]` instead of `&Vec<T>` in function parameters
- Use `Vec::with_capacity()` when size is known
- Avoid unnecessary trait bounds on struct definitions

## Testing Requirements

### Coverage Target

Minimum: **95%** | Target: **100%**

### Test Categories

#### 1. Unit Tests

Located in `#[cfg(test)] mod tests` blocks within source files.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_creation_attaches_metadata() {
        let err = AppError::internal("test")
            .with_field(field::str("key", "value"));
        assert_eq!(err.metadata().len(), 1);
    }

    #[test]
    fn invalid_input_returns_bad_request() {
        let result = validate_input("");
        assert!(matches!(
            result.unwrap_err().kind,
            AppErrorKind::BadRequest
        ));
    }
}
```

#### 2. Integration Tests

Located in `tests/` directory.

```rust
use masterror::prelude::*;

#[test]
fn axum_integration_converts_app_error_to_response() {
    let err = AppError::not_found("resource missing");
    let response = axum::response::IntoResponse::into_response(err);
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
```

#### 3. Doctests

Every public API example SHALL compile and run:

```rust
/// ```
/// use masterror::AppError;
/// let err = AppError::unauthorized("token expired");
/// assert_eq!(err.kind, masterror::AppErrorKind::Unauthorized);
/// ```
```

### Test Principles

1. **Deterministic**: No time, network, or environment dependencies
2. **Isolated**: Tests do not share mutable state
3. **Fast**: Unit tests complete in milliseconds
4. **Comprehensive**: Cover happy paths, error paths, edge cases

### Error Path Testing

Every error variant SHALL be tested:

```rust
#[test]
fn constraint_violation_maps_to_conflict() {
    let db_err = simulate_unique_violation();
    let app_err: AppError = db_err.into();
    assert!(matches!(app_err.kind, AppErrorKind::Conflict));
}
```

### Running Tests

```bash
cargo test --all-features
cargo test --no-default-features
cargo test --test integration_http
cargo test --doc

MIRIFLAGS="-Zmiri-strict-provenance" cargo +nightly miri test
```

## Pull Request Process

### 1. Create Issue First

Before implementing features or fixes:

1. Search existing issues to avoid duplicates
2. Create issue describing problem/feature
3. Wait for maintainer acknowledgment (for large changes)

### 2. Branch Naming

Branch name SHALL be the issue number without prefixes:

```bash
git checkout -b 123
```

### 3. Implementation

Make changes following coding standards. Commit frequently with descriptive messages.

### 4. Pre-PR Checklist

Before submitting, verify:

```bash
cargo +nightly fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo bench --no-run --features benchmarks
cargo doc --no-deps --all-features
```

### 5. Cancel Previous Workflows

Before pushing, cancel in-progress workflows to save CI resources:

```bash
gh run list --branch $(git branch --show-current) --status in_progress \
    --json databaseId --jq '.[].databaseId' | xargs -I {} gh run cancel {}
```

### 6. Push and Create PR

```bash
git push -u origin 123
gh pr create --title "123" --body "$(cat <<'EOF'
## Summary

Brief description of changes.

## Changes

- Added X
- Fixed Y
- Updated Z

## Test Plan

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Benchmarks show no regression
- [ ] Documentation updated

Closes #123
EOF
)"
```

### 7. PR Review

- Respond to feedback promptly
- Push additional commits to address comments
- Request re-review after changes
- Do not force-push after initial review

### 8. Merge

Maintainers will merge after:
- All CI checks pass
- At least one approval
- No unresolved conversations

## Commit Message Format

```
#<issue> <type>: <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `perf`: Performance improvement
- `refactor`: Code restructuring without behavior change
- `test`: Adding or updating tests
- `docs`: Documentation changes
- `ci`: CI/CD changes
- `chore`: Maintenance tasks

### Examples

```
#42 feat: add async handler support

Implements async error handlers for Axum and Actix-web frameworks.
Handlers can now return async closures that resolve to AppError.

#89 fix: correct gRPC status mapping for timeout errors

Previously mapped to UNAVAILABLE, now correctly maps to DEADLINE_EXCEEDED
per gRPC specification.

#120 perf: optimize metadata serialization path

Reduces allocations in ProblemJson conversion by 40%.
Benchmark results show 200ns improvement on average.
```

## Code Review Guidelines

### For Contributors

- Keep PRs focused (one issue per PR)
- Respond to feedback within 48 hours
- Accept constructive criticism professionally
- Ask questions if feedback is unclear

### For Reviewers

Review for:
1. **Correctness**: Does it solve the problem?
2. **Testing**: Adequate test coverage?
3. **Performance**: Any obvious inefficiencies?
4. **Documentation**: Public APIs documented?
5. **Style**: Follows project conventions?
6. **Safety**: No unsafe code without justification?

Use these labels:
- "looks good to me" (LGTM): Approve
- "request changes": Blocking issues
- "comment": Non-blocking suggestions

## Performance Benchmarking

### Running Benchmarks

```bash
cargo bench --features benchmarks --bench error_paths
cargo bench --features benchmarks --bench error_paths -- --save-baseline main
```

### Adding Benchmarks

Place criterion benchmarks in `benches/`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use masterror::AppError;

fn benchmark_error_creation(c: &mut Criterion) {
    c.bench_function("create_error_with_metadata", |b| {
        b.iter(|| {
            black_box(AppError::internal("test")
                .with_field(field::str("key", "value")))
        });
    });
}

criterion_group!(benches, benchmark_error_creation);
criterion_main!(benches);
```

### Regression Policy

CI will fail if benchmarks show >10% performance regression. If regression is necessary:
1. Document rationale in PR description
2. Add `[perf-regression-justified]` label
3. Require maintainer approval

## Documentation Standards

### README Updates

Update README.md when adding:
- New features requiring examples
- Breaking changes requiring migration
- New feature flags

### CHANGELOG

Add entries to CHANGELOG.md under "Unreleased" section:

```markdown
## [Unreleased]

### Added
- Support for async error handlers (#42)

### Fixed
- Incorrect gRPC status mapping for timeouts (#89)

### Changed
- BREAKING: Renamed `ErrorContext` to `Context` (#105)

### Performance
- Reduced metadata serialization allocations by 40% (#120)
```

### Wiki and Guides

Extended documentation goes in [wiki](https://github.com/RAprogramm/masterror/wiki):
- Step-by-step tutorials
- Migration guides
- Comparison with alternatives
- Troubleshooting recipes

## Issue Reporting

### Bug Reports

Use the bug report template and include:
1. Minimal reproducible example
2. Expected vs actual behavior
3. Environment (Rust version, OS, features enabled)
4. Error messages and stack traces

### Feature Requests

Use the feature request template and include:
1. Use case description
2. Proposed API (if applicable)
3. Alternatives considered
4. Impact assessment

### Questions

For usage questions:
1. Check existing documentation and issues first
2. Use GitHub Discussions (if enabled) or issues
3. Provide context and code samples

## Release Process

(For maintainers)

1. Update CHANGELOG.md with version number and date
2. Update version in all `Cargo.toml` files
3. Run full CI locally
4. Create git tag: `git tag -a v0.x.y -m "Release v0.x.y"`
5. Push tag: `git push origin v0.x.y`
6. GitHub Actions will publish to crates.io
7. Create GitHub release with CHANGELOG excerpt

## Getting Help

- **Documentation**: https://docs.rs/masterror
- **Issues**: https://github.com/RAprogramm/masterror/issues
- **Email**: andrey.rozanov.vl@gmail.com

## License

By contributing, you agree that your contributions will be dual-licensed under MIT OR Apache-2.0.
