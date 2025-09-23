use masterror::{AppCode, AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
#[error("oops")]
#[masterror(code = AppCode::Internal, category = AppErrorKind::Internal, redact())]
struct EmptyRedact;

fn main() {}
