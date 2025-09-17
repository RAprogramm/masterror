use masterror::Error;

#[derive(Debug, Error)]
#[error("{left:?} - {right:?}")]
struct BadStruct {
    #[from]
    left: DummyError,
    right: DummyError,
}

#[derive(Debug, Error)]
#[error("dummy")] 
struct DummyError;

fn main() {}
