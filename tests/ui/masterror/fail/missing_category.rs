use masterror::{AppCode, Masterror};

#[derive(Debug, Masterror)]
#[error("oops")]
#[masterror(code = AppCode::Internal)]
struct MissingCategory;

fn main() {}
