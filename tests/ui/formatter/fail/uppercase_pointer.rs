use masterror::Error;

#[derive(Debug, Error)]
#[error("{value:P}")]
struct UppercasePointer {
    value: *const u8,
}

fn main() {}
