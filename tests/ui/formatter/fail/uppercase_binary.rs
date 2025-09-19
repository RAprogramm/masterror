use masterror::Error;

#[derive(Debug, Error)]
#[error("{value:B}")]
struct UppercaseBinary {
    value: u8,
}

fn main() {}
