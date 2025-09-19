use masterror::Error;

#[derive(Debug, Error)]
#[error("invalid backtrace field")]
struct InvalidBacktrace {
    #[backtrace]
    trace: String
}

fn main() {}
