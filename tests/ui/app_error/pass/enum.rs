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

    let backend = ApiError::Backend;
    let app_backend: AppError = backend.into();
    assert!(matches!(app_backend.kind, AppErrorKind::Service));
    assert!(app_backend.message.is_none());

    let code: AppCode = ApiError::Backend.into();
    assert_eq!(code, AppCode::Service);
}
