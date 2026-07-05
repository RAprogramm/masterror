// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
enum ApiError {
    #[error("missing resource {id}")]
    #[app_error(
        kind = AppErrorKind::NotFound,
        code = AppCode::NotFound,
        message
    )]
    Missing { id: u64 },
    #[error("backend unavailable")]
    #[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
    Backend,
}

fn main() {
    let missing = ApiError::Missing { id: 7 };
    let app_missing: AppError = missing.into();
    assert!(matches!(app_missing.kind, AppErrorKind::NotFound));
    assert_eq!(app_missing.message.as_deref(), Some("missing resource 7"));
    assert!(app_missing.source_ref().is_some());
    let backend = ApiError::Backend;
    let app_backend: AppError = backend.into();
    assert!(matches!(app_backend.kind, AppErrorKind::Service));
    assert!(app_backend.message.is_none());
    assert!(app_backend.source_ref().is_some());
    let code: AppCode = ApiError::Backend.into();
    assert_eq!(code, AppCode::Service);
}
