use masterror::{AppCode, AppErrorKind, Error};

#[derive(Debug, Error)]
enum MissingCode {
    #[error("with code")]
    #[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound)]
    WithCode,
    #[error("without code")]
    #[app_error(kind = AppErrorKind::Service)]
    WithoutCode,
}

fn main() {}
