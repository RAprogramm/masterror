use masterror::{AppCode, AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
enum Mixed {
    #[error("with")]
    #[masterror(code = AppCode::Internal, category = AppErrorKind::Internal)]
    With,
    #[error("missing")]
    Missing
}

fn main() {}
