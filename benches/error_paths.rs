// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use core::{
    net::{IpAddr, Ipv4Addr},
    time::Duration
};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    hint::black_box
};

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use masterror::{AppError, AppErrorKind, Context, FieldRedaction, ProblemJson, ResultExt, field};

#[derive(Debug)]
struct DummyError;

impl Display for DummyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("dummy")
    }
}

impl std::error::Error for DummyError {}

fn context_into_error(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_into_error");

    group.bench_function("non_redacted", |b| {
        b.iter(|| {
            let context = build_context(false);
            let err = promote_error(context);
            black_box(err)
        });
    });

    group.bench_function("redacted", |b| {
        b.iter(|| {
            let context = build_context(true);
            let err = promote_error(context);
            black_box(err)
        });
    });

    group.finish();
}

fn problem_json_from_app_error(c: &mut Criterion) {
    let mut group = c.benchmark_group("problem_json_from_app_error");

    group.bench_function("non_redacted", |b| {
        b.iter_batched(
            || promote_error(build_context(false)),
            |error| {
                let problem = ProblemJson::from_app_error(error);
                black_box(problem);
            },
            BatchSize::SmallInput
        );
    });

    group.bench_function("redacted", |b| {
        b.iter_batched(
            || promote_error(build_context(true)),
            |error| {
                let problem = ProblemJson::from_app_error(error);
                black_box(problem);
            },
            BatchSize::SmallInput
        );
    });

    group.finish();
}

fn build_context(redacted: bool) -> Context {
    let mut context = Context::new(AppErrorKind::Service)
        .with(field::str("operation", "sync_job"))
        .with(field::u64("attempt", 3))
        .with(field::duration("elapsed", Duration::from_millis(275)))
        .with(field::bool("idempotent", true))
        .with(field::ip("peer", IpAddr::from(Ipv4Addr::LOCALHOST)));

    if redacted {
        context = context
            .with(field::str("token", "secret-token"))
            .redact_field("token", FieldRedaction::Hash)
            .redact(true)
            .track_caller();
    } else {
        context = context.with(field::str("token", "secret-token"));
    }

    context
}

fn promote_error(context: Context) -> AppError {
    let failing: Result<(), DummyError> = Err(DummyError);
    match failing.ctx(|| context) {
        Err(err) => err,
        Ok(_) => AppError::internal("benchmark expected error")
    }
}

criterion_group!(benches, context_into_error, problem_json_from_app_error);
criterion_main!(benches);
