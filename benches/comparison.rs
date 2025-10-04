// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Comparative benchmarks between masterror, thiserror, and anyhow.
//!
//! These benchmarks measure key error handling operations to demonstrate
//! that masterror provides competitive performance while offering additional
//! features like structured metadata and transport mappings.

use core::hint::black_box;
use std::io::Error as IoError;

use anyhow::Context as AnyhowContext;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

#[derive(Debug, thiserror::Error)]
#[error("IO operation failed: {source}")]
struct ThiserrorError {
    #[source]
    source: IoError
}

#[derive(Debug, masterror::Error)]
#[error("IO operation failed: {source}")]
#[app_error(kind = masterror::AppErrorKind::Internal, code = masterror::AppCode::Internal)]
struct MasterrorError {
    #[source]
    source: IoError
}

fn bench_error_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_creation");

    group.bench_function("thiserror", |b| {
        b.iter(|| {
            let err = ThiserrorError {
                source: IoError::other("disk offline")
            };
            black_box(err)
        });
    });

    group.bench_function("anyhow", |b| {
        b.iter(|| {
            let err = anyhow::anyhow!("IO operation failed");
            black_box(err)
        });
    });

    group.bench_function("masterror", |b| {
        b.iter(|| {
            let err = masterror::AppError::internal("IO operation failed");
            black_box(err)
        });
    });

    group.bench_function("masterror_with_source", |b| {
        b.iter(|| {
            let err = masterror::AppError::internal("IO operation failed")
                .with_context(IoError::other("disk offline"));
            black_box(err)
        });
    });

    group.finish();
}

fn bench_error_with_context(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_with_context");

    group.bench_function("anyhow_context", |b| {
        b.iter(|| {
            let result: Result<(), IoError> = Err(IoError::other("disk offline"));
            let err = result.context("IO operation failed").unwrap_err();
            black_box(err)
        });
    });

    group.bench_function("masterror_context", |b| {
        b.iter(|| {
            let err = masterror::AppError::internal("IO operation failed")
                .with_context(IoError::other("disk offline"));
            black_box(err)
        });
    });

    group.finish();
}

fn bench_error_chain_traversal(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_chain_traversal");

    group.bench_function("anyhow_chain", |b| {
        b.iter_batched(
            || {
                let io_err = IoError::other("disk offline");
                anyhow::Error::new(io_err).context("DB connection failed")
            },
            |err| {
                let chain_len = err.chain().count();
                black_box(chain_len)
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("masterror_chain", |b| {
        b.iter_batched(
            || {
                masterror::AppError::internal("DB connection failed")
                    .with_context(IoError::other("disk offline"))
            },
            |err| {
                let chain_len = err.chain().count();
                black_box(chain_len)
            },
            BatchSize::SmallInput
        );
    });

    group.finish();
}

fn bench_root_cause(c: &mut Criterion) {
    let mut group = c.benchmark_group("root_cause");

    group.bench_function("anyhow", |b| {
        b.iter_batched(
            || {
                let io_err = IoError::other("disk offline");
                anyhow::Error::new(io_err).context("DB connection failed")
            },
            |err| {
                let root_str = err.root_cause().to_string();
                black_box(root_str)
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("masterror", |b| {
        b.iter_batched(
            || {
                masterror::AppError::internal("DB connection failed")
                    .with_context(IoError::other("disk offline"))
            },
            |err| {
                let root_str = err.root_cause().to_string();
                black_box(root_str)
            },
            BatchSize::SmallInput
        );
    });

    group.finish();
}

fn bench_is_type_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_type_check");

    group.bench_function("anyhow", |b| {
        b.iter_batched(
            || {
                let io_err = IoError::other("disk offline");
                anyhow::Error::new(io_err)
            },
            |err| {
                let is_io = err.is::<IoError>();
                black_box(is_io)
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("masterror", |b| {
        b.iter_batched(
            || masterror::AppError::internal("error").with_context(IoError::other("disk offline")),
            |err| {
                let is_io = err.is::<IoError>();
                black_box(is_io)
            },
            BatchSize::SmallInput
        );
    });

    group.finish();
}

fn bench_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("display");

    group.bench_function("thiserror", |b| {
        b.iter_batched(
            || ThiserrorError {
                source: IoError::other("disk offline")
            },
            |err| {
                let s = err.to_string();
                black_box(s)
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("anyhow", |b| {
        b.iter_batched(
            || anyhow::anyhow!("IO operation failed"),
            |err| {
                let s = err.to_string();
                black_box(s)
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("masterror", |b| {
        b.iter_batched(
            || masterror::AppError::internal("IO operation failed"),
            |err| {
                let s = err.to_string();
                black_box(s)
            },
            BatchSize::SmallInput
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_error_creation,
    bench_error_with_context,
    bench_error_chain_traversal,
    bench_root_cause,
    bench_is_type_check,
    bench_display
);
criterion_main!(benches);
