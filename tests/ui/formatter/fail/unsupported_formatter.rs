use masterror::Error;

#[derive(Debug, Error)]
#[error("{value:y}")]
struct UnsupportedFormatter {
    value: u32,
}

fn main() {}
