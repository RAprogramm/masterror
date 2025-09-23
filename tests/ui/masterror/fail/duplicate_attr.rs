use masterror::{AppCode, AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
#[error("dup")]
#[masterror(code = AppCode::Internal, category = AppErrorKind::Internal)]
#[masterror(code = AppCode::Internal, category = AppErrorKind::Internal)]
struct Duplicate;

fn main() {}
