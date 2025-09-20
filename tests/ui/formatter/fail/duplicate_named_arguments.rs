use masterror::Error;

#[derive(Debug, Error)]
#[error("{value}", value = self.value, value = self.value)]
struct DuplicateNamedArgumentError {
    value: &'static str,
}

fn main() {}
