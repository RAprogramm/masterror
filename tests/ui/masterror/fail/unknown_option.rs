use masterror::{AppCode, AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
#[error("oops")]
#[masterror(code = AppCode::Internal, category = AppErrorKind::Internal, unknown)]
struct Unknown;

fn main() {}
