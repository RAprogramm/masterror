use masterror::Error;

#[derive(Debug, Error)]
#[error(transparent, code = 42)]
struct TransparentWithArgs(#[from] std::io::Error);

fn main() {}
