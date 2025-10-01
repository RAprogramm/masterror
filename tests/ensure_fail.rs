// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use core::sync::atomic::{AtomicUsize, Ordering};

use masterror::{AppError, AppErrorKind, AppResult};

static CALLS: AtomicUsize = AtomicUsize::new(0);

#[test]
fn ensure_allows_success_path() {
    fn run(flag: bool) -> AppResult<&'static str> {
        masterror::ensure!(flag, AppError::bad_request("flag required"));
        Ok("ok")
    }

    assert_eq!(run(true).unwrap(), "ok");
}

#[test]
fn ensure_yields_error_once() {
    fn build_error() -> AppError {
        CALLS.fetch_add(1, Ordering::SeqCst);
        AppError::service("bounded")
    }

    fn run(flag: bool) -> AppResult<()> {
        masterror::ensure!(cond = flag, else = build_error());
        Ok(())
    }

    CALLS.store(0, Ordering::SeqCst);
    assert!(run(false).is_err());
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);

    CALLS.store(0, Ordering::SeqCst);
    assert!(run(true).is_ok());
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
}

#[test]
fn ensure_preserves_error_kind() {
    fn run(flag: bool) -> AppResult<()> {
        masterror::ensure!(flag, AppError::unauthorized("token expired"));
        Ok(())
    }

    let err = run(false).unwrap_err();
    assert!(matches!(err.kind, AppErrorKind::Unauthorized));
}

#[test]
fn fail_returns_error() {
    fn run() -> AppResult<()> {
        masterror::fail!(AppError::forbidden("admin only"));
    }

    let err = run().unwrap_err();
    assert!(matches!(err.kind, AppErrorKind::Forbidden));
}

#[derive(Debug, PartialEq, Eq)]
struct CustomError(&'static str);

type CustomResult<T> = Result<T, CustomError>;

#[test]
fn macros_work_with_custom_error_types() {
    fn guard(flag: bool) -> CustomResult<&'static str> {
        masterror::ensure!(flag, CustomError("custom failure"));
        Ok("ok")
    }

    fn bail() -> CustomResult<()> {
        masterror::fail!(CustomError("fail"));
    }

    assert_eq!(guard(true).unwrap(), "ok");
    assert_eq!(guard(false).unwrap_err(), CustomError("custom failure"));
    assert_eq!(bail().unwrap_err(), CustomError("fail"));
}
