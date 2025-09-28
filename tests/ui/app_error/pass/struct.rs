use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("missing flag: {name}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
struct MissingFlag {
    name: &'static str,
}

fn main() {
    let err = MissingFlag { name: "feature" };
    let app: AppError = err.into();
    assert!(matches!(app.kind, AppErrorKind::BadRequest));
    assert_eq!(app.message.as_deref(), Some("missing flag: feature"));

    let code: AppCode = MissingFlag { name: "other" }.into();
    assert_eq!(code, AppCode::BadRequest);
}
