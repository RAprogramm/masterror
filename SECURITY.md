<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Version | Supported          | MSRV  | Status      |
| ------- | ------------------ | ----- | ----------- |
| 0.24.x  | :white_check_mark: | 1.90  | Active      |
| 0.23.x  | :x:                | 1.90  | Deprecated  |
| < 0.23  | :x:                | 1.70+ | Deprecated  |

Security patches are released as patch versions (e.g., 0.24.19 → 0.24.20) for the actively supported version line.

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue in masterror, please report it responsibly.

### Reporting Process

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. **Email** security reports to: **andrey.rozanov.vl@gmail.com**
3. **Include** the following information:
   - Description of the vulnerability
   - Steps to reproduce or proof-of-concept
   - Affected versions (if known)
   - Potential impact assessment
   - Suggested fix (if available)

### Response Timeline

- **Acknowledgment**: Within 48 hours of report
- **Initial assessment**: Within 5 business days
- **Fix development**: Varies by severity (critical: <7 days, high: <14 days)
- **Coordinated disclosure**: After patch is released and users have time to update

### Security Update Process

1. Vulnerability confirmed and assessed
2. Fix developed in private repository
3. Patch released as new version
4. Security advisory published on GitHub
5. Disclosure made after users have time to update (typically 7-14 days)

## Security Best Practices

When using masterror in production:

### 1. Dependency Management

- **Use latest stable version**: Security fixes are not backported to old versions
- **Enable Dependabot**: Automatically detect vulnerable dependencies
- **Run cargo-audit**: Regularly check for known vulnerabilities

```bash
cargo install cargo-audit
cargo audit
```

### 2. Error Message Redaction

- **Never expose sensitive data** in error messages without redaction
- **Use redaction policies** for PII (Personally Identifiable Information)

```rust
use masterror::{AppError, field, RedactionPolicy};

let err = AppError::internal("auth failed")
    .with_field(field::str_redacted(
        "user_email",
        email,
        RedactionPolicy::Hash
    ));
```

### 3. Production Configuration

- **Disable backtraces** in production (avoids leaking internal paths)
- **Limit metadata fields** (avoid exposing internal state)
- **Use message redaction** for user-facing errors

```toml
[dependencies]
masterror = { version = "0.24", features = ["std", "axum"], default-features = false }
```

### 4. Source Error Handling

- **Avoid exposing third-party error details** to end users
- **Log detailed errors** internally, return sanitized errors to clients

```rust
match db.query().await {
    Ok(result) => Ok(result),
    Err(e) => {
        // Log full error internally
        tracing::error!("Database error: {:?}", e);

        // Return sanitized error to client
        Err(AppError::service("database unavailable"))
    }
}
```

### 5. Supply Chain Security

- **Verify crate signatures**: Use cargo-vet for supply chain verification
- **Review dependencies**: Audit transitive dependencies periodically
- **Pin versions**: Use exact versions in Cargo.lock for reproducible builds

## Known Security Considerations

### 1. Error Source Chains

Error source chains may contain sensitive information from third-party libraries. Always review what is exposed to end users.

### 2. Metadata Serialization

Metadata fields are serialized in responses. Ensure redaction policies are correctly applied before exposing errors via HTTP/gRPC.

### 3. Backtrace Leakage

Backtraces can reveal internal file paths and stack frames. Disable the `backtrace` feature in production or implement custom filtering.

### 4. Timing Attacks

Error handling paths should not reveal timing information that could be exploited (e.g., database existence checks via different error latencies). This is application-specific and not handled by masterror.

## Security Audits

- **Internal audit**: Continuous review by maintainers
- **cargo-audit**: Automated dependency vulnerability scanning in CI
- **cargo-deny**: License and advisory checking in CI
- **Community review**: Open source code reviewed by users

## Security-Related Features

### No Unsafe Code

masterror contains **zero unsafe code**, eliminating entire classes of memory safety vulnerabilities:

```toml
[lints.rust]
unsafe_code = "forbid"
```

### Send + Sync Guarantees

All error types implement `Send + Sync`, ensuring safe concurrent usage without data races.

### Type Safety

- Strongly typed error kinds (no string matching)
- Compile-time validation of transport mappings
- No type confusion between error categories

## Vulnerability Disclosure History

No security vulnerabilities have been reported to date.

Future disclosures will be listed here with:
- CVE identifier (if applicable)
- Affected versions
- Severity rating
- Mitigation steps
- Fixed in version

## Security Contact

**Primary contact**: andrey.rozanov.vl@gmail.com

**PGP key**: Available on request for encrypted communication

## Additional Resources

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Error Handling Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Error_Handling_Cheat_Sheet.html)
- [Cargo Security Advisories](https://rustsec.org/)
