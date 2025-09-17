use masterror::Error;

#[derive(Debug, Error)]
enum BadEnum {
    #[error("{0} - {1}")]
    #[from]
    Two(#[source] DummyError, DummyError),
}

#[derive(Debug, Error)]
#[error("dummy")]
struct DummyError;

fn main() {}
