<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

# Performance Benchmarks

This document contains comparative benchmarks between `masterror`, `thiserror`, and `anyhow`.

## Running Benchmarks

```sh
cargo bench --bench comparison
cargo bench --bench error_paths
```

## Comparative Results

All benchmarks were run on the same machine with Criterion.rs default settings.

### Error Creation

| Implementation | Time (ns) | vs masterror |
|----------------|-----------|--------------|
| anyhow | 23.3 | 28% faster |
| **masterror** | **29.9** | baseline |
| thiserror | 33.7 | 13% slower |

**Analysis**: masterror is faster than thiserror for simple error creation while adding structured metadata capabilities.

### Error with Context

| Implementation | Time (ns) | vs masterror |
|----------------|-----------|--------------|
| anyhow | 54.0 | 48% faster |
| **masterror** | **104.2** | baseline |

**Analysis**: masterror is slower due to structured metadata and telemetry support. This is expected given the additional features.

### Error Chain Traversal

| Implementation | Time (ns) | vs masterror |
|----------------|-----------|--------------|
| anyhow | 120.6 | 17% faster |
| **masterror** | **144.9** | baseline |

**Analysis**: Comparable performance for chain iteration with minimal overhead.

### Root Cause Lookup

| Implementation | Time (ns) | vs masterror |
|----------------|-----------|--------------|
| anyhow | 138.8 | 1% faster |
| **masterror** | **140.9** | baseline |

**Analysis**: Virtually identical performance - differences within measurement noise.

### Type Checking (`is::<E>()`)

| Implementation | Time (ns) | vs masterror |
|----------------|-----------|--------------|
| anyhow | 88.4 | 35% faster |
| **masterror** | **136.6** | baseline |

**Analysis**: anyhow has optimization for downcasting. masterror overhead is acceptable for structured error handling.

### Display Formatting

| Implementation | Time (ns) | vs masterror |
|----------------|-----------|--------------|
| anyhow | 73.7 | 6% faster |
| **masterror** | **78.5** | baseline |
| thiserror | 132.7 | 69% slower |

**Analysis**: masterror performs comparably to anyhow and significantly better than thiserror.

## Conclusion

masterror provides **competitive performance** with thiserror and anyhow while offering unique features:

- ✅ Structured metadata with typed fields
- ✅ HTTP/gRPC transport mappings
- ✅ Redaction policies for sensitive data
- ✅ RFC7807 Problem JSON support
- ✅ Telemetry integration (tracing, metrics)

The performance overhead is minimal and justified by the additional functionality that would otherwise require manual implementation.

## Feature Comparison

| Feature | thiserror | anyhow | masterror |
|---------|-----------|--------|-----------|
| Derive macros | ✅ | ❌ | ✅ |
| Error chaining | ✅ | ✅ | ✅ |
| Context API | ❌ | ✅ | ✅ |
| Structured metadata | ❌ | ❌ | ✅ |
| HTTP mappings | ❌ | ❌ | ✅ |
| gRPC support | ❌ | ❌ | ✅ |
| Redaction | ❌ | ❌ | ✅ |
| Telemetry | ❌ | ❌ | ✅ |
| Zero-cost | ✅ | ❌ | ⚠️ |

**Legend**: ✅ Yes | ❌ No | ⚠️ Minimal overhead

## When to Use Each

- **thiserror**: Library errors that need zero runtime cost
- **anyhow**: Application errors with quick prototyping
- **masterror**: Production APIs needing structured, observable errors with transport mappings
