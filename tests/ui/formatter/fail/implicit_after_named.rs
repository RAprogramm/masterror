use masterror::Error;

#[derive(Debug, Error)]
#[error("{0}: {}", self.first, self.second)]
struct ImplicitAfterNamedError {
    first:  &'static str,
    second: &'static str,
}

fn main() {}
