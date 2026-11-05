// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Benchmarks for the transport-facing hot paths: building `AppError`
//! values with metadata, mapping them to wire responses and serializing the
//! resulting payloads.

use core::str::FromStr;
use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use masterror::{AppCode, AppError, ErrorResponse, ProblemJson, field};

fn build_rich_error() -> AppError {
    AppError::service("upstream service unavailable")
        .with_code(AppCode::Service)
        .with_field(field::str("operation", "sync_job"))
        .with_field(field::u64("attempt", 3))
        .with_field(field::bool("idempotent", true))
        .with_retry_after_secs(30)
        .with_www_authenticate("Bearer realm=\"api\"")
}

fn app_error_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("app_error_builder");
    group.bench_function("simple", |b| {
        b.iter(|| black_box(AppError::not_found(black_box("user not found"))));
    });
    group.bench_function("with_metadata", |b| {
        b.iter(|| black_box(build_rich_error()));
    });
    group.finish();
}

fn app_code_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("app_code_parse");
    group.bench_function("builtin", |b| {
        b.iter(|| black_box(AppCode::from_str(black_box("NOT_FOUND"))));
    });
    group.bench_function("custom", |b| {
        b.iter(|| black_box(AppCode::from_str(black_box("PAYMENT_DECLINED"))));
    });
    group.finish();
}

fn error_response_mapping(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_response_mapping");
    group.bench_function("from_ref", |b| {
        let error = build_rich_error();
        b.iter(|| black_box(ErrorResponse::from(black_box(&error))));
    });
    group.bench_function("from_owned", |b| {
        b.iter_batched(
            build_rich_error,
            |error| black_box(ErrorResponse::from(error)),
            BatchSize::SmallInput
        );
    });
    group.finish();
}

fn serialize_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_json");
    group.bench_function("error_response", |b| {
        let response = ErrorResponse::from(&build_rich_error());
        b.iter(|| black_box(serde_json::to_vec(black_box(&response)).unwrap()));
    });
    group.bench_function("problem_json", |b| {
        let problem = ProblemJson::from_app_error(build_rich_error());
        b.iter(|| black_box(serde_json::to_vec(black_box(&problem)).unwrap()));
    });
    group.finish();
}

fn display_render(c: &mut Criterion) {
    let error = build_rich_error();
    c.bench_function("display_render/app_error", |b| {
        b.iter(|| black_box(black_box(&error).to_string()));
    });
}

criterion_group!(
    benches,
    app_error_builder,
    app_code_parse,
    error_response_mapping,
    serialize_json,
    display_render
);
criterion_main!(benches);
