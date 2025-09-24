use masterror::{AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
#[error("oops")]
#[masterror(category = AppErrorKind::Internal)]
struct MissingCode;

fn main() {}
