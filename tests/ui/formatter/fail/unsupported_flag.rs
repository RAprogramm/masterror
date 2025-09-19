use masterror::Error;

#[derive(Debug, Error)]
#[error("{value:##x}")]
struct UnsupportedFlag {
    value: u32,
}

fn main() {}
