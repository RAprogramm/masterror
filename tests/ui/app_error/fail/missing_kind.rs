use masterror::Error;

#[derive(Debug, Error)]
#[error("oops")]
#[app_error(message)]
struct MissingSpec;

fn main() {}
